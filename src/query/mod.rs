use eframe::glow::Query;
use tantivy::collector::TopDocs;
use tantivy::query::{BooleanQuery, FuzzyTermQuery, Occur, QueryParser};
use tantivy::schema::Schema;
use tantivy::{Document, Index, IndexReader, ReloadPolicy, TantivyError, Term};

use crate::desktop_files::application_file::ApplicationFile;
use crate::desktop_files::desktop_file_search::application_files;

use self::search_index::index;

mod search_index;

pub struct Searcher {
    results_count: usize,
    query_parser: QueryParser,
    index: Index,
    tantivy_searcher: tantivy::Searcher,
    schema: Schema,
}

impl Searcher {
    pub fn new() -> Result<Self, String> {
        let (index, reader) = application_file_search_reader()?;
        let schema = index.schema();
        Ok(Self {
            results_count: 50,
            tantivy_searcher: reader.searcher(),
            query_parser: QueryParser::for_index(
                &index,
                vec![
                    schema.get_field("name").unwrap(),
                    schema.get_field("generic_name").unwrap(),
                    schema.get_field("comment").unwrap(),
                    schema.get_field("keywords").unwrap(),
                    schema.get_field("categories").unwrap(),
                ],
            ),
            index,
            schema,
        })
    }

    pub fn search(&self, search: &String) -> Result<Vec<String>, TantivyError> {
        let parsed_query = self.query_parser.parse_query(search)?;

        let fuzzy_query = FuzzyTermQuery::new(Term::from_field_text(
            self.schema.get_field("full_text").unwrap(),
            search
        ), 2, true);

        let query = BooleanQuery::new(vec![
            (Occur::Should, Box::from(fuzzy_query)),
            (Occur::Should, parsed_query),
        ]);

        let top_docs = self
            .tantivy_searcher
            .search(&query, &TopDocs::with_limit(self.results_count))?;

        println!("top_docs is: {:?}", top_docs);

        top_docs
            .into_iter()
            .map(|result| {
                self.tantivy_searcher.doc(result.1).and_then(|doc| {
                    println!("{:?}", doc);
                    Ok(doc
                        .get_first(self.schema.get_field("name").unwrap())
                        .unwrap()
                        .as_text()
                        .unwrap()
                        .to_string())
                })
            })
            .collect()
    }

    pub(crate) fn get_default_results(&self) -> Result<Vec<String>, TantivyError> {
        self.search(&"".into())
    }
}

fn index_size(index: &Index) -> Result<usize, String> {
    let size = index
        .load_metas()
        .map_err(|_| "Error loading index metadata".to_string())?
        .segments
        .iter()
        .map(|segment| segment.num_docs() as usize)
        .fold(0, |a, e| a + e);

    Ok(size)
}

fn write_application_files(index: &Index) -> Result<(), String> {
    let schema = index.schema();

    let mut writer = index
        .writer(50_000_000)
        .map_err(|_| "Could not create index writer")?;

    for application_file in application_files() {
        if let Some(doc) = application_file_to_document(application_file, &schema) {
            match writer.add_document(doc) {
                Ok(_) => {}
                Err(e) => return Err(e.to_string()),
            }
        } else {
            eprintln!("Could not convert application file to Tantivy document");
        }
    }

    writer.commit().map_err(|e| e.to_string())?;

    Ok(())
}

fn application_file_search_reader() -> Result<(Index, IndexReader), String> {
    let index = index()?;

    if index_size(&index)? == 0 {
        println!("Writing application files...");
        write_application_files(&index)?
    }

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()
        .map_err(|_| "Could not build index reader".to_string())?;

    Ok((index, reader))
}

fn application_file_to_document(app: ApplicationFile, schema: &Schema) -> Option<Document> {
    let mut doc = Document::default();

    let name = app.app_name;
    let comment = app.app_comment.unwrap_or_default();
    let generic_name = app.app_generic_name.unwrap_or_default();
    let exec = app.app_exec.unwrap_or_default();
    let keywords = app.app_keywords.map(|x| x.join(";")).unwrap_or_default();
    let categories = app.app_categories.map(|x| x.join(";")).unwrap_or_default();
    let mime_types = app.app_mime_types.map(|x| x.join(";")).unwrap_or_default();

    doc.add_text(
        schema.get_field("try_exec").unwrap(),
        app.app_try_exec
            .map(|x| x.to_string_lossy().to_string())
            .unwrap_or_default(),
    );
    doc.add_text(schema.get_field("exec").unwrap(), &exec);

    doc.add_text(schema.get_field("name").unwrap(), &name);
    doc.add_text(schema.get_field("generic_name").unwrap(), &generic_name);
    doc.add_text(schema.get_field("comment").unwrap(), &comment);

    doc.add_text(schema.get_field("keywords").unwrap(), &keywords);
    doc.add_text(schema.get_field("categories").unwrap(), &categories);
    doc.add_text(schema.get_field("mime_types").unwrap(), &mime_types);
    doc.add_text(
        schema.get_field("icon").unwrap(),
        app.app_icon.unwrap_or_default(),
    );

    doc.add_text(
        schema.get_field("full_text").unwrap(),
        name + "\t"
            + comment.as_str()
            + "\t"
            + generic_name.as_str()
            + "\t"
            + exec.as_str()
            + "\t"
            + keywords.as_str()
            + "\t"
            + categories.as_str()
            + "\t"
            + mime_types.as_str(),
    );

    Some(doc)
}
