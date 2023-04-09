use std::cmp::max;
use std::io::{self, stdout};

use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::{input::TermRead, event::Key};
use termion::event::Event;

use crate::{terminal::Terminal, config::Config};

pub struct Position{
    pub row: usize,
    pub col: usize,
}

pub struct Editor {
    terminal: Terminal,
    config: Config,
    curr_pos: Position,
}

impl Editor {
    pub(crate) fn new(config: Config) -> Self {
        Editor {
            terminal: Terminal::new(),
            config: config,
            curr_pos: Position { row: 1, col: 1 },
        }
    }

    pub(crate) fn run(&mut self) {
        self.terminal.enter_alternate_screen();

        let stdin = io::stdin();
        let mut _stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

        self.update();
        for c in stdin.events() {
            match c.unwrap(){
                Event::Key(Key::Char('q')) => {break;},
                Event::Key(key) => {
                    self.process_key(key);
                },
                Event::Mouse(mouse_event) => {
                    self.process_mouse_event(mouse_event);
                },
                Event::Unsupported(_) => {},
            }
            // println!("{:?}", c);
        }

        self.terminal.leave_alternate_screen();
    }

    fn process_key(&mut self, key: Key) {
        match key{
            Key::Left => self.col_left(),
            Key::Right => self.col_right(),
            Key::Up => self.row_up(),
            Key::Down => self.row_down(),
            _ => todo!("unknown key"),
        }
        self.update();
    }

    fn process_mouse_event(&mut self, mouse_event: termion::event::MouseEvent) {
        match mouse_event{
            termion::event::MouseEvent::Press(button, _col, _row) => {
                match button{
                    termion::event::MouseButton::Left => {},
                    termion::event::MouseButton::Right => {},
                    termion::event::MouseButton::Middle => {},
                    termion::event::MouseButton::WheelUp => self.row_up(),
                    termion::event::MouseButton::WheelDown => self.row_down(),
                }
            },
            termion::event::MouseEvent::Release(_, _) => {},
            termion::event::MouseEvent::Hold(_, _) => {},
        }
        self.update();
    }

    fn update(&mut self) {
        self.terminal.sync_terminal_size();
        self.terminal.clear();
        self.terminal.set_cursor_pos(1, 1);

        self.render();

        self.terminal.set_cursor_pos(self.curr_pos.row, self.curr_pos.col);
        self.terminal.flush();
    }

    fn render(&mut self) {
        let height = max(3, self.terminal.size.height);
        let mut frames: Vec<String> = Vec::with_capacity(height);

        frames.push("1 Welcome to Lemon".to_string());

        // add doc lines
        for i in 2..(height-2){
            frames.push(format!("doc line {}", i));
        }

        frames.push(format!("status line ({}/{}) ({}/{})", self.curr_pos.row, self.terminal.size.height, self.curr_pos.col, self.terminal.size.width));
        frames.push("command line".to_string());

        self.terminal.println(frames.join("\r\n"));
    }

    fn col_left(&mut self) {
        if self.curr_pos.col>1{
            self.curr_pos.col=self.curr_pos.col.saturating_sub(1);
        }
    }

    fn col_right(&mut self) {
        if self.curr_pos.col<self.terminal.size.width{
            self.curr_pos.col=self.curr_pos.col.saturating_add(1);
        }
    }

    fn row_up(&mut self) {
        if self.curr_pos.row>1{
            self.curr_pos.row=self.curr_pos.row.saturating_sub(1);
        }
    }

    fn row_down(&mut self) {
        if self.curr_pos.row<self.terminal.size.height{
            self.curr_pos.row=self.curr_pos.row.saturating_add(1);
        }
    }
}
