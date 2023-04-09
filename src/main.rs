mod editor;
mod terminal;
mod config;

use editor::Editor;
use config::Config;

fn main() {
    let config = Config::new();
    let mut editor = Editor::new(config);
    editor.run();
}
