use crate::models::Coordinates;

pub fn translate_game_coords_to_board_coords(coordinates: Coordinates) -> Coordinates {
    Coordinates {
        x: coordinates.x * 4 + 1,
        y: coordinates.y * 2 + 1,
    }
}
