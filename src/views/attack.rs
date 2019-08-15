use std::io::{Stdout, Write};
use termion::cursor::Goto;
use termion::raw::RawTerminal;
use termion::{color, style};

use crate::{
    models::{
        Coordinates,
        Attack,
        AttackResult
    },
    views::{
        utils::{
            translate_game_coords_to_board_coords
        }
    }
};

pub struct AttackView {
    origin: Coordinates,
    model: Attack
}

impl AttackView {
    pub fn new(origin: Coordinates, model: Attack) -> AttackView {
        AttackView {
            origin,
            model
        }
    }

    pub fn render(&self, stdout: &mut RawTerminal<Stdout>) {
        let symbol = match self.model.result {
            AttackResult::Hit => "X",
            AttackResult::Miss => "^"
        };

        let board_coords = translate_game_coords_to_board_coords(self.model.coordinates);
        let screen_coords = Coordinates {
            x: board_coords.x + self.origin.x,
            y: board_coords.y + self.origin.y
        };

        write!(
            stdout,
            "{}{} {} {}",
            Goto(screen_coords.x, screen_coords.y),
            color::Bg(color::Blue),
            symbol,
            style::Reset
        ).unwrap();
    }
}