use crate::{
    models::{Coordinates, Ship},
    views::utils::translate_game_coords_to_board_coords,
};
use std::io::{Stdout, Write};
use termion::cursor::Goto;
use termion::raw::RawTerminal;
use termion::{color, style};

pub struct ShipView {
    origin: Coordinates,
    model: Ship,
}

impl ShipView {
    pub fn new(origin: Coordinates, model: Ship) -> ShipView {
        ShipView { origin, model }
    }

    pub fn update(self, model: Ship) -> ShipView {
        ShipView { model, ..self }
    }

    pub fn render(&self, stdout: &mut RawTerminal<Stdout>) {
        for segment in self.model.segments.iter() {
            let board_coords = translate_game_coords_to_board_coords(segment.coordinates);
            let screen_coords = Coordinates {
                x: board_coords.x + self.origin.x,
                y: board_coords.y + self.origin.y,
            };

            write!(
                stdout,
                "{}{}   {}",
                Goto(screen_coords.x, screen_coords.y),
                color::Bg(color::Red),
                style::Reset
            )
            .unwrap();
        }
    }
}
