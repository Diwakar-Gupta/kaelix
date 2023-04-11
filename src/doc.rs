use std::{cmp::min, fs};

use regex::Regex;
use std::cmp::max;
use termion::{
    color,
    event::{Key, MouseEvent},
    style,
};

use crate::{common::Position, common::Size, config::Config};

pub struct StatusLine {
    pub typee: Type,
    pub text: String,
}

pub enum Type {
    Error,
    Warning,
    Info,
}

struct Rectangle {
    row: (usize, usize),
    col: (usize, usize),
}

pub struct Doc {
    pub lines: Vec<String>,
    pub cursor_pos: Position,
    status_line: StatusLine,
    pub offset: Position,
}

impl Doc {
    pub fn new() -> Self {
        let lines = vec!["This is starting doc".to_string()];
        Self {
            lines,
            cursor_pos: Position { row: 0, col: 0 },
            status_line: StatusLine {
                text: "New file".to_string(),
                typee: Type::Info,
            },
            offset: Position { row: 0, col: 0 },
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
                status_line: StatusLine {
                    typee: Type::Info,
                    text: "File opened".to_string(),
                },
                offset: Position { row: 0, col: 0 },
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
}

impl Doc {
    pub fn process_key(&mut self, key: &Key) {
        match key {
            Key::Left => {
                self.col_left();
            }
            Key::Right => {
                self.col_right();
            }
            Key::Up => {
                self.row_up();
            }
            Key::Down => {
                self.row_down();
            }
            Key::Char(ch) => self.write_char(ch.to_owned()),
            Key::Backspace => self.handle_backspace(),
            _ => todo!(),
        }
    }

    fn col_left(&mut self) {
        if self.cursor_pos.col > 0 {
            self.cursor_pos.col = self.cursor_pos.col.saturating_sub(1);
        } else {
            if self.cursor_pos.row > 0 {
                self.cursor_pos.row = self.cursor_pos.row.saturating_sub(1);
                self.cursor_pos.col = max(self.lines[self.cursor_pos.row].len(), 0);
            }
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
    fn write_char(&mut self, ch: char) {
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
    }
    fn handle_backspace(&mut self) {
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
