use interface::window::open_window;
use query::Searcher;

mod interface;
mod query;
mod config;
mod desktop_files;

fn main() {
    let query = Searcher::new().unwrap();
    open_window(query);
}