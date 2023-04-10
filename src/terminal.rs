use std::{fmt::Display, io::stdout, io::Write};

use termion::{
    raw::IntoRawMode,
    screen::{IntoAlternateScreen, ToAlternateScreen, ToMainScreen},
};

use crate::common::Size;

pub struct Terminal {
    pub size: Size,
    screen: termion::screen::AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>,
}

impl Terminal {
    pub fn new() -> Self {
        let screen = stdout()
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap();
        let size = termion::terminal_size().unwrap();

        Self {
            screen,
            size: Size {
                height: size.1 as usize,
                width: size.0 as usize,
            },
        }
    }
    pub fn print(&mut self, message: impl Display) {
        // print!("{}", message);
        write!(self.screen, "{}", message).unwrap();
    }
    pub fn print_flush(&mut self, message: impl Display) {
        self.print(message);
        self.flush();
    }
    pub fn set_cursor_pos(&mut self, row: usize, col: usize) {
        self.print(format!("{}", termion::cursor::Goto(col as u16, row as u16)));
    }
    pub fn leave_alternate_screen(&mut self) {
        write!(self.screen, "{}", ToMainScreen).unwrap();
    }
    pub fn enter_alternate_screen(&mut self) {
        write!(self.screen, "{}", ToAlternateScreen).unwrap();
    }
    pub fn clear(&mut self) {
        self.print(termion::clear::All);
    }

    pub fn sync_terminal_size(&mut self) {
        let size = termion::terminal_size().unwrap();

        self.size.width = size.0 as usize;
        self.size.height = size.1 as usize;
    }

    pub(crate) fn flush(&mut self) {
        self.screen.flush().unwrap();
    }
}
