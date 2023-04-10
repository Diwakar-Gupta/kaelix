mod common;
mod config;
mod doc;
mod editor;
mod filetree;
mod status_line;
mod terminal;

use std::env::args;

use config::Config;
use editor::Editor;

fn main() {
    let args: Vec<String> = args().collect();
    let file_path = if args.len() == 2 {
        Some(args[1].clone())
    } else {
        None
    };
    let config = Config::new();
    let mut editor = Editor::new(config, file_path);
    editor.run();
}
