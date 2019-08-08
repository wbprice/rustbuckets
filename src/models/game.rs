use crate::{
    models::{
        Ship,
        Attack,
        Scores,
        Faction
    }
};

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