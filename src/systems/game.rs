use crate::{
    entities::{
        Ship,
        Attack
    }
};

#[derive(Debug, Default)]
pub struct Scores {
    hits: u16,
    misses: u16
}

#[derive(Debug)]
pub enum Faction {
    Blue,
    Red
}

impl Default for Faction {
    fn default() -> Self {
        Faction::Blue
    }
}

#[derive(Debug, Default)]
pub struct Game {
    pub blue_score: Scores,
    pub red_score: Scores,
    pub blue_ships: Box<[Ship]>,
    pub red_ships: Box<[Ship]>,
    pub blue_attacks: Box<[Attack]>,
    pub red_attacks: Box<[Attack]>,
    pub active_player: Faction
}