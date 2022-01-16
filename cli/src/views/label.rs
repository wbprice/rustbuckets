use crate::models::{Coordinates, Label};
use std::io::{Stdout, Write};
use termion::cursor::Goto;
use termion::raw::RawTerminal;
use termion::{color, style};

pub struct LabelView {
    origin: Coordinates,
    model: Label,
}

impl LabelView {
    pub fn new(origin: Coordinates, model: Label) -> LabelView {
        LabelView { origin, model }
    }

    pub fn render(&self, stdout: &mut RawTerminal<Stdout>) {
        write!(
            stdout,
            "{}{}{}{}",
            Goto(self.origin.x, self.origin.y),
            color::Fg(color::White),
            self.model.content,
            style::Reset
        )
        .unwrap();
    }
}
