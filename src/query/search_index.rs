use tantivy::directory::MmapDirectory;
use tantivy::schema::*;
use tantivy::Index;

pub fn index() -> Result<Index, String> {
    let dir = MmapDirectory::open(
        crate::config::index_directory().ok_or("Couldn't load index directory")?,
    ).map_err(|e| e.to_string())?;
    Index::open_or_create(dir, schema()).map_err(|e| e.to_string())
}

pub fn schema() -> Schema {
    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field("name", TEXT | STORED);
    schema_builder.add_text_field("try_exec", TEXT | STORED);
    schema_builder.add_text_field("exec", TEXT | STORED);

    schema_builder.add_text_field("generic_name", TEXT);
    schema_builder.add_text_field("comment", TEXT);

    schema_builder.add_text_field("keywords", TEXT);
    schema_builder.add_text_field("categories", TEXT);
    schema_builder.add_text_field("mime_types", TEXT);

    schema_builder.add_text_field("icon", TEXT | STORED);
    
    schema_builder.add_text_field("full_text", TEXT);

    schema_builder.build()
}
