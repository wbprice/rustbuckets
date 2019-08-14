use std::io::{Stdout, Write};
use termion::cursor::Goto;
use termion::raw::{RawTerminal};
use termion::{color, style};

use crate::models::{Board, Coordinates};

pub struct BoardView {
    pub origin: Coordinates,
    model: Board,
}

impl BoardView {
    pub fn new(origin: Coordinates, model: Board) -> BoardView {
        BoardView { origin, model }
    }

    pub fn update(self, model: Board) -> BoardView {
        BoardView { model, ..self }
    }

    fn render_latitude_line(&self, stdout: &mut RawTerminal<Stdout>) {
        let mut output = "+".to_string();
        for _ in 0..self.model.height {
            output.push_str("---+");
        }
        write!(
            stdout,
            "{}{}{}{}\n\r",
            color::Fg(color::White),
            color::Bg(color::Blue),
            output,
            style::Reset
        )
        .unwrap();
    }

    fn render_longitude_line(&self, stdout: &mut RawTerminal<Stdout>) {
        let mut output = "|".to_string();
        for _ in 0..self.model.width {
            output.push_str("   |");
        }
        write!(
            stdout,
            "{}{}{}{}\n\r",
            color::Fg(color::White),
            color::Bg(color::Blue),
            output,
            style::Reset
        )
        .unwrap();
    }

    pub fn render(&self, stdout: &mut RawTerminal<Stdout>) {
        write!(stdout, "{}", Goto(self.origin.x, self.origin.y)).unwrap();
        for _ in 1..self.model.height + 1 {
            self.render_latitude_line(stdout);
            self.render_longitude_line(stdout);
        }
        self.render_latitude_line(stdout);
        write!(stdout, "\n\r").unwrap();
    }
}
