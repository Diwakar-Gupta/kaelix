use std::cmp::max;
use std::io::{self, stdout};

use termion::event::Event;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::style;
use termion::{event::Key, input::TermRead};

use crate::common::{Position, Size, Task};
use crate::doc::Doc;
use crate::filetree::FileTree;
use crate::status_line::{InputStatus, StatusLine};
use crate::{config::Config, terminal::Terminal};

#[allow(dead_code)]
enum FocusComponent {
    Doc,
    FileTree,
}

#[allow(dead_code)]
enum View {
    Doc,
    FileTree,
    Both(FocusComponent),
}

pub struct Editor {
    active_doc: usize,
    config: Config,
    #[allow(dead_code)]
    file_tree: FileTree,
    cursor_pos: Position,
    docs: Vec<Doc>,
    terminal: Terminal,
    status_input_active: bool,
    status_line: StatusLine,
    view: View,
}

impl Editor {
    pub(crate) fn new(config: Config, file_path: Option<String>) -> Self {
        let mut docs = vec![];

        let doc = if let Some(path) = file_path {
            match Doc::open(&path) {
                Some(doc) => doc,
                None => {
                    let mut doc = Doc::new();
                    doc.set_file_path(path);
                    doc.set_command_status("New named doc created".to_string());
                    doc
                }
            }
        } else {
            Doc::new()
        };
        docs.push(doc);

        Editor {
            docs,
            active_doc: 0,
            terminal: Terminal::new(),
            config,
            cursor_pos: Position { row: 1, col: 1 },
            status_input_active: false,
            view: View::Doc,
            file_tree: FileTree {},
            status_line: StatusLine::new(),
        }
    }

    pub(crate) fn run(&mut self) {
        self.terminal.enter_alternate_screen();

        let stdin = io::stdin();
        let mut _stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

        self.update();
        for c in stdin.events() {
            let task: Task = match c.unwrap() {
                Event::Key(Key::Ctrl('q')) => {
                    break;
                }
                Event::Key(key) => self.process_key_event(key),
                Event::Mouse(mouse_event) => self.process_mouse_event(mouse_event),
                Event::Unsupported(_) => {
                    self.terminal.leave_alternate_screen();
                    todo!("Unsupported event")
                }
            };
            self.process_task(task);
            if self.docs.is_empty() {
                break;
            }
            self.update();
        }

        self.terminal.leave_alternate_screen();
    }

    fn process_key_event(&mut self, key: Key) -> Task {
        if self.status_input_active {
            let command = self.status_line.process_key(&key);
            match command {
                InputStatus::Processing => Task::None,
                InputStatus::Cancelled => {
                    self.status_input_active = false;
                    Task::SetCommand("Cancelled".to_string())
                }
                InputStatus::Done(input) => {
                    self.status_input_active = false;
                    self.process_command_event(input)
                }
            }
        } else {
            match self.view {
                View::Doc | View::Both(FocusComponent::Doc) => {
                    self.docs[self.active_doc].process_key(&key)
                }
                View::FileTree => todo!(),
                View::Both(_) => todo!(),
            }
        }
    }

    fn process_mouse_event(&mut self, mouse_event: termion::event::MouseEvent) -> Task {
        match self.view {
            View::Doc | View::Both(FocusComponent::Doc) => {
                self.docs[self.active_doc].process_mouse_event(&mouse_event);
            }
            View::FileTree => todo!(),
            View::Both(FocusComponent::FileTree) => todo!(),
        };
        Task::None
    }

    fn process_command_event(&mut self, input: String) -> Task {
        match self.view {
            View::Doc | View::Both(FocusComponent::Doc) => {
                self.docs[self.active_doc].process_command_input(input)
            }
            View::FileTree | View::Both(FocusComponent::FileTree) => todo!(),
        }
    }

    fn update_cursor_from_curr_doc(&mut self) {
        let col_offset = self.docs[self.active_doc].get_line_number_length();

        let col_offset = match self.view {
            View::Doc => col_offset + self.config.general.line_number_padding_right,
            View::FileTree => todo!(),
            View::Both(FocusComponent::Doc) => {
                self.config.general.file_tree_width
                    + col_offset
                    + self.config.general.line_number_padding_right
            }
            View::Both(FocusComponent::FileTree) => todo!(),
        };

        let doc_cursor = self.docs[self.active_doc].cursor_pos;
        let doc_offset = self.docs[self.active_doc].offset;

        let row_offset: usize = 1;

        self.cursor_pos = Position {
            row: row_offset + doc_cursor.row + 1 - doc_offset.row,
            col: col_offset + doc_cursor.col + 1 - doc_offset.col,
        };
    }

    fn update(&mut self) {
        self.terminal.sync_terminal_size();
        self.terminal.clear();
        self.terminal.set_cursor_pos(1, 1);

        self.render();

        self.terminal
            .set_cursor_pos(self.cursor_pos.row, self.cursor_pos.col);
        self.terminal.flush();
    }

    fn update_cursor_pos(&mut self) {
        // update cursor based on view after render
        if self.status_input_active {
            self.update_cursor_from_command_line();
        } else {
            match self.view {
                View::Doc | View::Both(FocusComponent::Doc) => self.update_cursor_from_curr_doc(),
                View::FileTree | View::Both(FocusComponent::FileTree) => todo!(),
            }
        }
    }

    fn update_cursor_from_command_line(&mut self) {
        self.cursor_pos.row = self.terminal.size.height;
        self.cursor_pos.col = self.status_line.get_cursor_with_prefix();
    }

    fn render(&mut self) {
        let mut frames: Vec<String> = Vec::with_capacity(max(3, self.terminal.size.height));

        frames.push(self.render_header());

        let mut doc_frame = self.get_sub_frame();
        frames.append(&mut doc_frame);

        // should be after sub frame rendering to get correct cursor & offset
        frames.push(self.render_status_line());

        let command_render = self.render_command_line();
        frames.push(command_render);

        self.terminal.print(frames.join("\r\n"));
    }

    fn render_header(&self) -> String {
        match self.view {
            View::Doc | View::Both(FocusComponent::Doc) => self.render_doc_tabs(),
            View::FileTree | View::Both(FocusComponent::FileTree) => todo!(),
        }
    }

    fn render_doc_tabs(&self) -> String {
        let mut tabs = String::new();

        for (i, doc) in self.docs.iter().enumerate() {
            let title = doc.get_title();

            if i == self.active_doc {
                tabs.push_str(format!(" {} |", title).as_str());
            } else {
                tabs.push_str(format!(" {}{} |{}", style::Faint, title, style::Reset).as_str());
            }
        }
        tabs
    }

    fn render_command_line(&mut self) -> String {
        if !self.status_line.is_taking_input() {
            match self.view {
                View::Doc | View::Both(FocusComponent::Doc) => self
                    .status_line
                    .set_status(self.docs[self.active_doc].command_status.clone()),
                View::FileTree | View::Both(FocusComponent::FileTree) => todo!(),
            };
        }
        self.status_line.render()
    }

    fn render_status_line(&mut self) -> String {
        self.update_cursor_pos();

        match self.view {
            View::Doc | View::Both(FocusComponent::Doc) => {
                let active_doc = &self.docs[self.active_doc];
                let status = format!(
                    "{}/{} | {}",
                    active_doc.cursor_pos.row + 1,
                    active_doc.lines.len(),
                    active_doc.cursor_pos.col,
                );
                format!(
                    "{}{}",
                    " ".repeat(self.terminal.size.width.saturating_sub(status.len()) - 1),
                    status,
                )
            }
            _ => {
                format!(
                    "status line ({}/{}) ({}/{})",
                    self.cursor_pos.row,
                    self.terminal.size.height,
                    self.cursor_pos.col,
                    self.terminal.size.width
                )
            }
        }
    }

    fn get_sub_frame(&mut self) -> Vec<String> {
        match self.view {
            View::Doc => self.render_doc_view(),
            View::FileTree => self.render_file_tree_view(),
            View::Both(_) => todo!("both rendering needs to be implemented"),
        }
    }

    fn render_doc_view(&mut self) -> Vec<String> {
        self.render_doc(Size {
            height: max(3, self.terminal.size.height) - 3,
            width: self.terminal.size.width,
        })
    }
    fn render_file_tree_view(&self) -> Vec<String> {
        todo!()
    }

    fn render_doc(&mut self, size: Size) -> Vec<String> {
        let mut frame = self.docs[self.active_doc].render(&size, &self.config);

        while frame.len() < size.height {
            frame.push("~".to_string());
        }

        frame
    }
}

// file related operations
impl Editor {
    fn new_document(&mut self) {
        // Create a new document
        self.docs.push(Doc::new());
        self.active_doc = self.docs.len() - 1;
        self.view = match self.view {
            View::Doc => View::Doc,
            View::FileTree => View::Doc,
            View::Both(_) => View::Both(FocusComponent::Doc),
        }
    }
    fn open_document(&mut self, file: Option<String>) {
        match file {
            Some(path) => {
                // File was specified
                if let Some(doc) = Doc::open(&path) {
                    self.docs.push(doc);
                    self.active_doc = self.docs.len() - 1;
                    self.view = match self.view {
                        View::Doc => View::Doc,
                        View::FileTree => View::Doc,
                        View::Both(_) => View::Both(FocusComponent::Doc),
                    }
                } else {
                }
            }
            None => {
                // Ask for a file and open it
                todo!()
            }
        }
    }
}

// process task
impl Editor {
    fn process_task(&mut self, task: Task) {
        match task {
            Task::SetCommand(text) => self.process_set_command(text),
            Task::AskInput(prefix) => self.ask_input(prefix),
            Task::None => {}
            Task::NewDoc => self.new_document(),
            Task::OpenDoc(path) => self.open_document(Some(path)),
            Task::NextTab => {
                self.active_doc = self.active_doc.saturating_add(1);
                if self.active_doc >= self.docs.len() {
                    self.active_doc = 0;
                }
            }
            Task::PrevTab => {
                if self.active_doc == 0 {
                    self.active_doc = self.docs.len() - 1;
                } else {
                    self.active_doc = self.active_doc.saturating_sub(1);
                }
            }
            Task::CloseCurrentTab => {
                if self.docs[self.active_doc].can_close() {
                    self.docs.remove(self.active_doc);
                    self.active_doc = self.active_doc.saturating_sub(1);
                } else {
                    self.status_line
                        .set_status("Can't close this tab".to_string());
                }
            }
        }
    }
    fn process_set_command(&mut self, text: String) {
        self.status_line.set_status(text);
    }
    fn ask_input(&mut self, prefix: String) {
        self.status_line.take_input(prefix);
        self.status_input_active = true;
    }
}
