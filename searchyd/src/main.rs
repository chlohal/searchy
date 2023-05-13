use std::sync::Arc;

use crate::window::open_window;


pub mod window;
mod unix_stream_sub;

fn main() {
    open_window(Arc::new(actions::load_db())).unwrap();
}