use termion::event::Key;

#[derive(PartialEq)]
enum Status {
    Input(String, String),
    Still(String),
}

impl Status {
    pub fn insert(&mut self, idx: usize, ch: char) {
        match self {
            Status::Input(_, text) => {
                text.insert(idx, ch);
            }
            Status::Still(_) => unimplemented!("command line is still, can't push_char"),
        }
    }

    fn handle_backspace(&mut self, idx: usize) {
        match self {
            Status::Input(_, text) => {
                text.remove(idx);
            }
            Status::Still(_) => unimplemented!("command line is still, can't backspace"),
        }
    }
}

pub struct StatusLine {
    status: Status,
    cursor: usize,
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
            cursor: 0,
        }
    }
    pub fn is_taking_input(&self) -> bool {
        match self.status {
            Status::Input(_, _) => true,
            _ => false,
        }
    }
    pub fn set_status(&mut self, text: String) {
        self.status = Status::Still(text);
    }
    pub fn render(&self) -> String {
        let line = match &self.status {
            Status::Input(prefix, input) => format!("{}: {}", prefix, input),
            Status::Still(mes) => mes.to_string(),
        };
        line
    }
    pub fn process_key(&mut self, key: &Key) -> InputStatus {
        match key {
            Key::Esc => InputStatus::Cancelled,
            Key::Char('\n') => match &self.status {
                Status::Input(_, input) => InputStatus::Done(input.to_string()),
                Status::Still(_) => unimplemented!(),
            },
            Key::Char(ch) => self.write_input(ch.to_owned()),
            Key::Backspace => self.handle_backspace(),
            Key::Left => {
                self.cursor = self.cursor.saturating_sub(1);
                InputStatus::Processing
            }
            Key::Right => {
                self.cursor = self.cursor.saturating_add(1);
                InputStatus::Processing
            }
            _ => InputStatus::Processing,
        }
    }

    pub fn get_cursor_with_prefix(&self) -> usize {
        match &self.status {
            Status::Input(p, _) => (p.len() + 2) + (self.cursor + 1),
            Status::Still(_) => unimplemented!(),
        }
    }

    fn write_input(&mut self, ch: char) -> InputStatus {
        self.status.insert(self.cursor, ch);
        self.cursor += 1;
        InputStatus::Processing
    }

    fn handle_backspace(&mut self) -> InputStatus {
        match &self.status {
            Status::Input(_, txt) => {
                if !txt.is_empty() && self.cursor > 0 {
                    self.status.handle_backspace(self.cursor - 1);
                    self.cursor -= 1;
                }
            }
            Status::Still(_) => {}
        }
        InputStatus::Processing
    }

    pub(crate) fn take_input(&mut self, prefix: String) {
        self.status = Status::Input(prefix, String::new());
        self.cursor = 0;
    }
}
