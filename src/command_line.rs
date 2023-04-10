use termion::event::Key;

pub struct CommandLine {}

pub enum CommandResult{
    Processing,
    Cancelled,
    Done(String)
}

impl CommandLine {
    pub fn render(&self) -> Vec<String> {
        vec!["This is command line".to_string()]
    }
}

impl CommandLine {
    pub fn process_key(&mut self, key: &Key) -> CommandResult {
        CommandResult::Done("command".to_string())
    }
}
