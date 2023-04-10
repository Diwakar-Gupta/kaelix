use std::cmp::max;
use std::io::{self, stdout};

use termion::event::Event;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::{event::Key, input::TermRead};

use crate::command_line::{CommandLine, CommandResult};
use crate::common::Size;
use crate::doc::Doc;
use crate::filetree::FileTree;
use crate::{config::Config, terminal::Terminal};

pub struct Position {
    pub row: usize,
    pub col: usize,
}

enum FocusComponent {
    Doc,
    FileTree,
}

enum View {
    Doc,
    FileTree,
    Both(FocusComponent),
}

pub struct Editor {
    active_doc: usize,
    config: Config,
    file_tree: FileTree,
    curr_pos: Position,
    docs: Vec<Doc>,
    terminal: Terminal,
    command_active: bool,
    command_line: CommandLine,
    view: View,
}

impl Editor {
    pub(crate) fn new(config: Config) -> Self {
        let docs = vec![Doc::new()];
        Editor {
            docs,
            active_doc: 0,
            terminal: Terminal::new(),
            config: config,
            curr_pos: Position { row: 1, col: 1 },
            command_active: false,
            view: View::Doc,
            file_tree: FileTree {},
            command_line: CommandLine {},
        }
    }

    pub(crate) fn run(&mut self) {
        self.terminal.enter_alternate_screen();

        let stdin = io::stdin();
        let mut _stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

        self.update();
        for c in stdin.events() {
            match c.unwrap() {
                Event::Key(Key::Char('q')) => {
                    break;
                }
                Event::Key(key) => {
                    self.process_key_event(key);
                }
                Event::Mouse(mouse_event) => {
                    self.process_mouse_event(mouse_event);
                }
                Event::Unsupported(_) => {}
            }
            // println!("{:?}", c);
        }

        self.terminal.leave_alternate_screen();
    }

    fn process_key_event(&mut self, key: Key) {
        if self.command_active {
            let command = self.command_line.process_key(&key);
            match command{
                CommandResult::Processing => todo!(),
                CommandResult::Cancelled => {
                    self.command_active = false;
                },
                CommandResult::Done(_) => todo!(),
            }
        } else {
            let should_editor_process = match self.view {
                View::Doc => self.docs[self.active_doc].process_key(&key),
                View::FileTree => todo!(),
                View::Both(_) => todo!(),
            };
            if should_editor_process {
                self.process_key(&key);
            }
        }

        self.update();
    }

    fn process_mouse_event(&mut self, mouse_event: termion::event::MouseEvent) {
        match mouse_event {
            termion::event::MouseEvent::Press(button, _col, _row) => match button {
                termion::event::MouseButton::Left => {}
                termion::event::MouseButton::Right => {}
                termion::event::MouseButton::Middle => {}
                termion::event::MouseButton::WheelUp => {
                    self.row_up();
                }
                termion::event::MouseButton::WheelDown => {
                    self.row_down();
                }
            },
            termion::event::MouseEvent::Release(_, _) => {}
            termion::event::MouseEvent::Hold(_, _) => {}
        }
        self.update();
    }

    fn update(&mut self) {
        self.terminal.sync_terminal_size();
        self.terminal.clear();
        self.terminal.set_cursor_pos(1, 1);

        self.render();

        self.terminal
            .set_cursor_pos(self.curr_pos.row, self.curr_pos.col);
        self.terminal.flush();
    }

    fn render(&mut self) {
        let mut frames: Vec<String> = Vec::with_capacity(max(3, self.terminal.size.height));

        frames.push("1 Welcome to Lemon".to_string());

        let sub_frame = self.get_sub_frame();

        let mut doc_frame = self.get_sub_frame();
        frames.append(&mut doc_frame);

        frames.push(format!(
            "status line ({}/{}) ({}/{})",
            self.curr_pos.row,
            self.terminal.size.height,
            self.curr_pos.col,
            self.terminal.size.width
        ));

        let mut command_render = self.command_line.render();
        assert_eq!(1, command_render.len());
        frames.append(&mut command_render);

        self.terminal.print(frames.join("\r\n"));
    }

    fn get_sub_frame(&self) -> Vec<String> {
        match self.view {
            View::Doc => self.render_doc_view(),
            View::FileTree => self.render_file_tree_view(),
            View::Both(_) => todo!("both rendering needs to be implemented"),
        }
    }

    fn render_doc_view(&self) -> Vec<String> {
        self.render_doc(Size {
            height: max(3, self.terminal.size.height) - 3,
            width: self.terminal.size.width,
        })
    }
    fn render_file_tree_view(&self) -> Vec<String> {
        todo!()
    }

    fn render_doc(&self, size: Size) -> Vec<String> {
        let mut frame = self.docs[self.active_doc].render(&size);

        while frame.len() < size.height {
            frame.push("~".to_string());
        }

        frame
    }
}

// handle key events
impl Editor {
    fn process_key(&mut self, key: &Key) {
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
            _ => todo!("unknown key"),
        }
    }

    fn col_left(&mut self) -> bool {
        if self.curr_pos.col > 1 {
            self.curr_pos.col = self.curr_pos.col.saturating_sub(1);
            true
        } else {
            false
        }
    }

    fn col_right(&mut self) -> bool {
        if self.curr_pos.col < self.terminal.size.width {
            self.curr_pos.col = self.curr_pos.col.saturating_add(1);
            true
        } else {
            false
        }
    }

    fn row_up(&mut self) -> bool {
        if self.curr_pos.row > 1 {
            self.curr_pos.row = self.curr_pos.row.saturating_sub(1);
            true
        } else {
            false
        }
    }

    fn row_down(&mut self) -> bool {
        if self.curr_pos.row < self.terminal.size.height {
            self.curr_pos.row = self.curr_pos.row.saturating_add(1);
            true
        } else {
            false
        }
    }
}
