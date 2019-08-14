use termion::raw::{RawTerminal};
use termion::cursor::Goto;
use termion::{color, style};
use std::io::{Stdout, Write};
use crate::{
    models::{
        Coordinates,
        Label
    }
};

pub struct LabelView<'a> {
    origin: Coordinates,
    model: &'a Label
}

impl<'a> LabelView<'a> {
    pub fn new(origin: Coordinates, model: &'a Label) -> LabelView {
        LabelView {
            origin,
            model
        }
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