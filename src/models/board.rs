use crate::models::{Coordinates};
use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    thread_rng, Rng,
};
use std::io::{stdin, stdout, Stdin, Stdout, Write};
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{color, style};
use std::{thread, time};

#[derive(Debug, Default)]
pub struct Board {
    width: u16,
    height: u16
}

impl Board {
    pub fn new(width: u16, height: u16) -> Board {
        Board {
            width,
            height
        }
    }

    fn render_latitude_line(&self, stdout: &mut RawTerminal<Stdout>) {
        let mut output = "+".to_string();
        for _ in 0..self.height {
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
        for _ in 0..self.width {
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

    pub fn render(&self, stdout: &mut RawTerminal<Stdout>, coordinates: Coordinates) {
        write!(stdout, "{}", Goto(coordinates.x, coordinates.y)).unwrap();
        for _ in 1..self.height + 1 {
            self.render_latitude_line(stdout);
            self.render_longitude_line(stdout);
        }
        self.render_latitude_line(stdout);
        write!(stdout, "\n\r").unwrap();
    }
}