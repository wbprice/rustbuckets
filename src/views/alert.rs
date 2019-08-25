use crate::{
    models::{
        Coordinates,
        Alert,
        Level
    }
};
use std::io::{Stdout, Write};
use termion::cursor::Goto;
use termion::raw::RawTerminal;
use termion::{color, style};

pub struct AlertView {
    pub origin: Coordinates,
    pub model: Alert
}

impl AlertView {
    pub fn new(origin: Coordinates, model: Alert) -> AlertView {
        AlertView {
            origin,
            model
        }
    }

    pub fn update(self, model: Alert) -> AlertView {
        AlertView {
            model,
            ..self
        }
    }

    fn draw_horizontal_edge(&self) -> String {
        let line : String = (1..48).map(|_| "-").collect();

        format!(
            "{}",
            line
        )
    }

    pub fn render(&self, stdout: &mut RawTerminal<Stdout>) {
        // Let's assume alerts are always 3 rows tall and 48 columns wide.
        let label = match self.model.level {
            Level::Info => "INFO",
            Level::Success => "SUCCESS",
            Level::Warning => "WARNING",
            Level::Error => "ERROR"
        };

        // WTF
        let color = match self.model.level {
            Level::Info => color::Fg(color::Cyan).to_string(),
            Level::Success => color::Fg(color::Green).to_string(),
            Level::Warning => color::Fg(color::Yellow).to_string(),
            Level::Error => color::Fg(color::Red).to_string()
        };

        write!(
            stdout,
            "{}{}{}{}{}{}{}{}",
            Goto(self.origin.x, self.origin.y),
            color,
            self.draw_horizontal_edge(),
            Goto(self.origin.x, self.origin.y + 1),
            format!("{}: {}", label, self.model.content),
            Goto(self.origin.x, self.origin.y + 2),
            self.draw_horizontal_edge(),
            style::Reset
        ).unwrap();
    }
}