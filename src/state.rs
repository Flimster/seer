use crate::digits::Digit;

use std::io::{Stdout, Write};

use termion::color;
use termion::raw::RawTerminal;
use termion::style;
use termion::terminal_size;

struct Position {
    x: u16,
    y: u16,
}

impl Position {
    fn new() -> Position {
        let (x, y) = terminal_size().expect("Failed to get terminal size");

        Position { x, y }
    }
}

pub struct Task<'a> {
    name: String,
    pos: Position,
    color: color::Fg<&'a dyn color::Color>,
}

impl<'a> Task<'a> {
    pub fn new(name: String, color: color::Fg<&'a dyn color::Color>) -> Task {
        let pos = Position::new();

        Task {
            name,
            pos,
            color,
        }
    }

    pub fn render(self, status: String, stdout: &mut RawTerminal<Stdout>) {
		let offset: u16 = ((self.name.len() + status.len()) / 2) as u16;
		let task = format!("{} - {}", self.name, status);
        write!(
            stdout,
            "{}{}{}{}{}{}",
            termion::cursor::Goto(self.pos.x / 2 - offset + 3, self.pos.y / 2 - 2),
            termion::style::Bold,
            self.color,
            task,
            color::Fg(color::Reset),
            style::Reset,
        )
        .unwrap();
        stdout.flush().unwrap();
    }
}

pub struct Timer {
    digits: Vec<Box<dyn Digit>>,
    pos: Position,
}

impl Timer {
    pub fn new(digits: Vec<Box<dyn Digit>>) -> Timer {
        let pos = Position::new();

        Timer { digits, pos }
    }

    pub fn render(self, mut stdout: &mut RawTerminal<Stdout>) {
        // hours
        self.digits[0].render((self.pos.x / 2 - 22, self.pos.y / 2), &mut stdout);
        self.digits[1].render((self.pos.x / 2 - 14, self.pos.y / 2), &mut stdout);
        // minutes
        self.digits[2].render((self.pos.x / 2 - 4, self.pos.y / 2), &mut stdout);
        self.digits[3].render((self.pos.x / 2 + 4, self.pos.y / 2), &mut stdout);
        // seconds
        self.digits[4].render((self.pos.x / 2 + 14, self.pos.y / 2), &mut stdout);
        self.digits[5].render((self.pos.x / 2 + 22, self.pos.y / 2), &mut stdout);
    }
}
