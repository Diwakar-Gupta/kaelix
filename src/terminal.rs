use std::{io::stdout, io::Write, fmt::Display};

use termion::{
    raw::IntoRawMode,
    screen::{IntoAlternateScreen, ToAlternateScreen, ToMainScreen},
};

pub struct Size{
    pub height: usize,
    pub width: usize,
}

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

        Self { screen, size: Size { height: size.1 as usize, width: size.0 as usize } }
    }
    pub fn print(&self, message: impl Display){
        print!("{}", message);
    }
    pub fn println(&self, message: impl Display){
        println!("{}", message);
    }
    pub fn set_cursor_pos(&self, row: usize, col: usize){
        self.print(format!("{}", termion::cursor::Goto(col as u16, row as u16)));
    }
    pub fn leave_alternate_screen(&mut self) {
        write!(self.screen, "{}", ToMainScreen).unwrap();
    }
    pub fn enter_alternate_screen(&mut self) {
        write!(self.screen, "{}", ToAlternateScreen).unwrap();
    }
    pub fn clear(&mut self){
        self.print(termion::clear::All);
    }

    pub fn sync_terminal_size(&mut self){
        let size = termion::terminal_size().unwrap();

        self.size.width = size.0 as usize;
        self.size.height = size.1 as usize;
    }

    pub(crate) fn flush(&self) {
        println!();
    }
}
