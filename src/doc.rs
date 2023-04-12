use std::{cmp::min, ffi::OsStr, fs, io, path::Path};

use regex::Regex;
use std::cmp::max;
use termion::{
    color,
    event::{Key, MouseEvent},
    style,
};

use crate::{
    common::Position,
    common::{Size, Task},
    config::Config,
};

enum TaskPending {
    SaveFile,
    None,
    OpenDoc,
}

pub struct Doc {
    pub lines: Vec<String>,
    pub cursor_pos: Position,
    pub offset: Position,
    file_path: Option<String>,
    task_pending: TaskPending,
    pub command_status: String,
}

impl Doc {
    pub fn new() -> Self {
        let lines = vec!["".to_string()];
        Self {
            lines,
            cursor_pos: Position { row: 0, col: 0 },
            offset: Position { row: 0, col: 0 },
            file_path: None,
            task_pending: TaskPending::None,
            command_status: "Untitled file".to_string(),
        }
    }

    pub fn get_line_number_length(&self) -> usize {
        self.lines.len().to_string().len()
    }

    pub fn render(&mut self, size: &Size, config: &Config) -> Vec<String> {
        let line_number_width = self.get_line_number_length();
        let content_width =
            size.width - line_number_width - config.general.line_number_padding_right;

        let rows_to_render = min(size.height, self.lines.len() - self.offset.row);
        let cols_to_render = content_width;

        let mut frames = Vec::with_capacity(rows_to_render);

        self.update_offset(rows_to_render, cols_to_render);

        let line_indexs_to_render = self.offset.row..(self.offset.row + rows_to_render);

        for (line, i) in self.lines[line_indexs_to_render.clone()]
            .iter()
            .zip(line_indexs_to_render)
        {
            let index = i + 1;
            let sub_line = Self::sub_string(line, self.offset.col, cols_to_render);
            assert!(sub_line.len() <= cols_to_render);
            frames.push(format!(
                "{}{}{}{}{}{}",
                " ".repeat(line_number_width - index.to_string().len()),
                color::Fg(color::Green),
                index,
                style::Reset,
                " ".repeat(config.general.line_number_padding_right),
                sub_line,
            ));
        }
        frames
    }

    fn sub_string(str: &str, start: usize, len: usize) -> String {
        let mut result = String::with_capacity(min(len, str.len().saturating_sub(start)));

        for ch in str.as_bytes().iter().skip(start).take(len) {
            let ch = char::try_from(ch.to_owned()).unwrap();
            result.push(ch);
        }

        result
    }

    pub(crate) fn open(path: &str) -> Option<Self> {
        if let Ok(file) = fs::read_to_string(path) {
            // File exists
            let mut file = Self::split_file(&file);
            if file.is_empty() {
                file.push("");
            }
            Some(Self {
                lines: file.iter().map(|row| row.to_string()).collect(),
                cursor_pos: Position { row: 0, col: 0 },
                offset: Position { row: 0, col: 0 },
                file_path: Some(path.to_string()),
                task_pending: TaskPending::None,
                command_status: "File loaded succesfully".to_string(),
            })
        } else {
            None
        }
    }
    pub fn split_file(contents: &str) -> Vec<&str> {
        // Detect DOS line ending
        let splitter = Regex::new("(?ms)(\r\n|\n)").unwrap();
        splitter.split(contents).collect()
    }

    pub(crate) fn can_close(&self) -> bool {
        true
    }
}

impl Doc {
    pub fn process_key(&mut self, key: &Key) -> Task {
        match key {
            Key::Left => {
                self.col_left();
                Task::None
            }
            Key::Right => {
                self.col_right();
                Task::None
            }
            Key::Up => {
                self.row_up();
                Task::None
            }
            Key::Down => {
                self.row_down();
                Task::None
            }
            Key::Char(ch) => self.write_char(ch.to_owned()),
            Key::Backspace => self.handle_backspace(),
            Key::Ctrl('s') => self.process_save_file(),
            Key::Ctrl('n') => Task::NewDoc,
            Key::Ctrl('o') => {
                self.task_pending = TaskPending::OpenDoc;
                Task::AskInput("Doc path".to_string())
            }
            Key::Ctrl('k') => Task::PrevTab,
            Key::Ctrl('l') => Task::NextTab,
            Key::Ctrl('w') => Task::CloseCurrentTab,
            _ => Task::None,
        }
    }

    fn col_left(&mut self) {
        if self.cursor_pos.col > 0 {
            self.cursor_pos.col = self.cursor_pos.col.saturating_sub(1);
        } else if self.cursor_pos.row > 0 {
            self.cursor_pos.row = self.cursor_pos.row.saturating_sub(1);
            self.cursor_pos.col = max(self.lines[self.cursor_pos.row].len(), 0);
        }
    }

    fn col_right(&mut self) {
        if self.cursor_pos.col < self.lines[self.cursor_pos.row].len() {
            self.cursor_pos.col = self.cursor_pos.col.saturating_add(1);
        } else if self.cursor_pos.row + 1 < self.lines.len() {
            self.cursor_pos.row = self.cursor_pos.row.saturating_add(1);
            self.cursor_pos.col = 0;
        }
    }

    fn row_up(&mut self) {
        if self.cursor_pos.row > 0 {
            self.cursor_pos.row = self.cursor_pos.row.saturating_sub(1);
            if self.lines[self.cursor_pos.row].len() <= self.cursor_pos.col {
                self.cursor_pos.col = max(self.lines[self.cursor_pos.row].len(), 0);
            }
        }
    }

    fn row_down(&mut self) {
        if self.cursor_pos.row + 1 < self.lines.len() {
            self.cursor_pos.row = self.cursor_pos.row.saturating_add(1);
            if self.lines[self.cursor_pos.row].len() <= self.cursor_pos.col {
                self.cursor_pos.col = max(self.lines[self.cursor_pos.row].len(), 0);
            }
        }
    }

    fn update_offset(&mut self, render_nrows: usize, render_ncols: usize) {
        let last_row = self.offset.row + render_nrows - 1;
        let last_col = self.offset.col + render_ncols - 1;

        if last_row < self.cursor_pos.row {
            self.offset.row += self.cursor_pos.row - last_row;
        }
        if last_col < self.cursor_pos.col {
            self.offset.col += self.cursor_pos.col - last_col;
        }
        if self.cursor_pos.row < self.offset.row {
            self.offset.row = self.cursor_pos.row;
        }
        if self.cursor_pos.col < self.offset.col {
            self.offset.col = self.cursor_pos.col;
        }
    }
}

// doc edit operations
impl Doc {
    pub fn set_file_path(&mut self, path: String) {
        self.file_path = Some(path);
    }
    fn write_char(&mut self, ch: char) -> Task {
        if ch == '\n' {
            let mut new_string = self.lines.remove(self.cursor_pos.row);
            new_string.insert(self.cursor_pos.col, '\n');

            let mut new_lines = Self::split_file(&new_string);
            assert_eq!(2, new_lines.len(), "Doc::split_file should return 2 lines");

            self.lines
                .insert(self.cursor_pos.row, new_lines.remove(0).to_string());
            self.lines
                .insert(self.cursor_pos.row + 1, new_lines.remove(0).to_string());

            self.cursor_pos.row += 1;
            self.cursor_pos.col = 0;
        } else {
            self.lines[self.cursor_pos.row].insert(self.cursor_pos.col, ch);
            self.col_right();
        }
        Task::None
    }
    fn handle_backspace(&mut self) -> Task {
        if self.cursor_pos.col == 0 {
            // merge self.cursor_pos.row-1 and self.cursor_pos.row
            if self.cursor_pos.row > 0 {
                let new_col = self.lines[self.cursor_pos.row - 1].len();
                let curr_line = self.lines.remove(self.cursor_pos.row);
                self.lines[self.cursor_pos.row - 1].push_str(curr_line.as_str());

                self.cursor_pos.row -= 1;
                self.cursor_pos.col = new_col;
            }
        } else {
            self.col_left();
            self.lines[self.cursor_pos.row].remove(self.cursor_pos.col);
        }
        Task::None
    }
    fn process_save_file(&mut self) -> Task {
        match self.file_path.as_ref() {
            Some(file_path) => match self.save_file(file_path) {
                Ok(_) => {
                    self.command_status = format!("File saved: {}", file_path);
                    Task::SetCommand(format!("File saved: {}", file_path))
                }
                Err(err) => {
                    self.command_status = format!("Unable to save file: {}", err);
                    Task::None
                }
            },
            None => {
                // ask for file path from command line
                self.task_pending = TaskPending::SaveFile;
                Task::AskInput("File path".to_string())
            }
        }
    }
    pub fn get_title(&self) -> String {
        match &self.file_path {
            Some(path) => Path::new(path)
                .file_name()
                .unwrap_or_else(|| OsStr::new(path))
                .to_str()
                .unwrap_or(path)
                .to_string(),
            None => "[No name]".to_string(),
        }
    }
    fn get_doc_content(&self) -> String {
        let mut content = String::new();

        for line in self.lines.iter() {
            content.push_str(line);
            content.push('\n');
        }
        content
    }
    fn save_file(&self, path: &str) -> io::Result<()> {
        let content = self.get_doc_content();

        fs::write(path, content)
    }
}

// handle mouse event
impl Doc {
    pub fn process_mouse_event(&mut self, mouse_event: &MouseEvent) {
        match mouse_event {
            MouseEvent::Press(button_event, _, _) => match button_event {
                termion::event::MouseButton::Left => {}
                termion::event::MouseButton::Right => {}
                termion::event::MouseButton::Middle => {}
                termion::event::MouseButton::WheelUp => self.row_up(),
                termion::event::MouseButton::WheelDown => self.row_down(),
            },
            MouseEvent::Release(_, _) => {}
            MouseEvent::Hold(_, _) => {}
        }
    }
}

// handle command input
impl Doc {
    pub fn set_command_status(&mut self, status: String) {
        self.command_status = status;
    }
    pub(crate) fn process_command_input(&mut self, input: String) -> Task {
        let task = match self.task_pending {
            TaskPending::SaveFile => {
                self.file_path = Some(input);
                self.task_pending = TaskPending::None;
                self.process_save_file()
            }
            TaskPending::None => unimplemented!("No task asking for input"),
            TaskPending::OpenDoc => Task::OpenDoc(input),
        };
        self.task_pending = TaskPending::None;
        task
    }
}
