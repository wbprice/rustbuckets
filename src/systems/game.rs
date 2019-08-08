use crate::{entities::Ship};

#[derive(Debug, Default)]
pub struct Scores {
    hits: u16,
    misses: u16
}

#[derive(Debug, Default)]
pub struct Game {
    pub blue_score: Scores,
    pub red_score: Scores,
    pub blue_ships: 
    pub red_ships: Scores,
    pub blue_attacks: Scores,


}