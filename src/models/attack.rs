use crate::{
    models::{
        Ship,
        Coordinates,
        AttackResult
    }
};

#[derive(Debug)]
pub struct Attack {
    pub coordinates: Coordinates,
    pub result: AttackResult
}

impl Attack {
    pub fn new(ships: &Vec<Ship>, coordinates: Coordinates) -> Attack {
        for ship in ships.iter() {
            for ship_segment in ship.segments.iter() {
                if ship_segment.coordinates.x == coordinates.x &&
                   ship_segment.coordinates.y == coordinates.y {
                       return Attack {
                           coordinates,
                           result: AttackResult::Hit
                       }
                   }
            }
        }
        Attack {
            coordinates,
            result: AttackResult::Miss
        }
    }
}