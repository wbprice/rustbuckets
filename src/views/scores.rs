use crate::{
    views::utils::{translate_game_coords_to_board_coords},
    models::{
        Coordinates,
        Scores
    }
};

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
            "Red Team".to_string(),
            color::Fg(color::White),
            Goto(self.origin.x, self.origin.y + 5),
            self.red_score.hits,
            Goto(self.origin.x, self.origin.y + 6),
            self.red_score.misses,
            style::Reset
        )
    }
}