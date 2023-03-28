use eframe::egui::{self, Button, ScrollArea};

use crate::query::Searcher;

pub fn open_window(searcher: Searcher) -> Result<(), eframe::Error> {
    let mut opts = eframe::NativeOptions::default();
    opts.decorated = false;
    eframe::run_native(
        "Searchy",
        opts,
        Box::new(|_| Box::new(SearchingWindow::with_searcher(searcher))),
    )
}

struct SearchingWindow {
    search_query: String,
    results: Vec<String>,
    searcher: Searcher
}

impl SearchingWindow {
    fn with_searcher(searcher: Searcher) -> Self {
        Self {
            search_query: "".to_string(),
            results: searcher.get_default_results().unwrap_or_default(),
            searcher
        }
    }
}

impl eframe::App for SearchingWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    for result in self.results.iter() {
                        ui.add(Button::new(result));
                    }
                });
            });

            let text = ui.text_edit_singleline(&mut self.search_query);

            if text.changed() {
                println!("Updated {}", self.search_query);
                match self.searcher.search(&self.search_query) {
                    Ok(res) => {self.results = res;}
                    Err(err) => { eprintln!("Error {}", err); }
                }
            }
        });
    }
}
