mod command_line;
mod common;
mod config;
mod doc;
mod editor;
mod filetree;
mod terminal;

use config::Config;
use editor::Editor;

fn main() {
    let config = Config::new();
    let mut editor = Editor::new(config);
    editor.run();
}
