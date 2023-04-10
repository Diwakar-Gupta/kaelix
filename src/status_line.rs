use termion::event::Key;

pub struct StatusLine {}

pub enum InputStatus{
    Processing,
    Cancelled,
    Done(String)
}

impl StatusLine {
    pub fn render(&self) -> Vec<String> {
        vec!["This is command line".to_string()]
    }
    pub fn process_key(&mut self, key: &Key) -> InputStatus {
        InputStatus::Done("command".to_string())
    }
}
