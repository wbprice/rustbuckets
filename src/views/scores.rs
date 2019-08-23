use crate::{
    models::{
        Coordinates,
        Scores
    }
};
use std::io::{Stdout, Write};
use termion::cursor::Goto;
use termion::raw::RawTerminal;
use termion::{color, style};

pub struct ScoresView {
    origin: Coordinates,
    model: Scores
}

impl ScoresView {
    pub fn new(origin: Coordinates, model: Scores) -> ScoresView {
        ScoresView { origin, model }
    }

    pub fn update(self, model: Scores) -> ScoresView {
        ScoresView { model, ..self }
    }

    pub fn render(&self, stdout: &mut RawTerminal<Stdout>) {
        writeln!(
            stdout,
            "{}{}{}{}{}Hits: {}{}Misses: {}{}",
            Goto(self.origin.x, self.origin.y + 4),
            color::Fg(color::Red),
            self.model.label.to_string(),
            color::Fg(color::White),
            Goto(self.origin.x, self.origin.y + 5),
            self.model.hits,
            Goto(self.origin.x, self.origin.y + 6),
            self.model.misses,
            style::Reset
        ).unwrap();
    }
}