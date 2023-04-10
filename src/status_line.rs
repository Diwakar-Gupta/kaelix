use termion::event::Key;

#[derive(PartialEq)]
enum Status {
    Input(String, String),
    Still(String),
}

pub struct StatusLine {
    status: Status,
}

pub enum InputStatus {
    Processing,
    Cancelled,
    Done(String),
}

impl StatusLine {
    pub fn new() -> Self {
        Self {
            status: Status::Still("This is command line".to_string()),
        }
    }
    pub fn render(&self) -> Vec<String> {
        let line = match &self.status {
            Status::Input(prefix, input) => format!("{}: {}", prefix, input),
            Status::Still(mes) => mes.to_string(),
        };
        vec![line]
    }
    pub fn process_key(&mut self, key: &Key) -> InputStatus {
        match key {
            Key::Esc => InputStatus::Cancelled,
            Key::Char('\n') => match &self.status {
                Status::Input(_, input) => InputStatus::Done(input.to_string()),
                Status::Still(_) => unimplemented!(),
            },
            _ => todo!(),
        }
    }

    pub(crate) fn take_input(&mut self, prefix: &str) {
        self.status = Status::Input(prefix.to_string(), String::new());
    }
}
