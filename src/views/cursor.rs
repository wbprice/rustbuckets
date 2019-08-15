use std::io::{Stdout, Write};
use termion::cursor::Goto;
use termion::raw::RawTerminal;
use termion::{color, style};

use crate::{
    models::{Coordinates, Cursor},
    views::utils::translate_game_coords_to_board_coords,
};

#[derive(Copy, Clone)]
pub struct CursorView {
    pub origin: Coordinates,
    pub model: Cursor,
}

impl CursorView {
    pub fn new(origin: Coordinates, model: Cursor) -> CursorView {
        CursorView { origin, model }
    }

    pub fn render(self, stdout: &mut RawTerminal<Stdout>) {
        let board_coords = translate_game_coords_to_board_coords(self.model.origin);
        let screen_coords = Coordinates {
            x: board_coords.x + self.origin.x,
            y: board_coords.y + self.origin.y,
        };
        write!(
            stdout,
            "{}{}[ ]{}",
            Goto(screen_coords.x, screen_coords.y),
            color::Bg(color::Blue),
            style::Reset
        )
        .unwrap()
    }

    pub fn update(self, model: Cursor) -> CursorView {
        CursorView { model, ..self }
    }
}
