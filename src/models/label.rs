use termion::raw::{RawTerminal};
use termion::cursor::Goto;
use termion::{color, style};
use std::io::{Stdout, Write};
use crate::models::{
    Coordinates
};

pub struct Label {
    content: String
}

impl Label {
    pub fn new(content: String) -> Label {
        Label {
            content
        }
    }

    pub fn render(&self, stdout: &mut RawTerminal<Stdout>, coordinates: Coordinates) {
        write!(
            stdout,
            "{}{}{}{}",
            Goto(coordinates.x, coordinates.y),
            color::Fg(color::White),
            self.content,
            style::Reset
        )
        .unwrap();
    }
}