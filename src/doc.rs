use std::cmp::min;

use termion::event::Key;

use crate::{common::Size, editor::Position};

pub struct Doc {
    lines: Vec<String>,
    cursor_pos: Position,
}

impl Doc {
    pub fn new() -> Self {
        let lines = vec!["This is starting doc".to_string()];
        Self {
            lines,
            cursor_pos: Position { row: 0, col: 0 },
        }
    }

    pub fn render(&self, size: &Size) -> Vec<String> {
        let mut frames = Vec::with_capacity(min(size.height, self.lines.len()));

        for (i, line) in self.lines.iter().enumerate() {
            frames.push(format!("{} {}", i, line));
        }
        frames
    }
}

impl Doc {
    pub fn process_key(&mut self, key: &Key) -> bool {
        true
    }
}
