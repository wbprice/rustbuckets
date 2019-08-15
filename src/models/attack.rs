use crate::models::{AttackResult, Coordinates, Ship};

#[derive(Debug)]
pub struct Attack {
    pub coordinates: Coordinates,
    pub result: AttackResult,
}

impl Attack {
    pub fn new(ships: &Vec<Ship>, coordinates: Coordinates) -> Attack {
        for ship in ships.iter() {
            for coords in ship.get_segment_coordinates().into_iter() {
                if coords.x == coordinates.x && coords.y == coordinates.y
                {
                    return Attack {
                        coordinates,
                        result: AttackResult::Hit,
                    };
                }
            }
        }
        Attack {
            coordinates,
            result: AttackResult::Miss,
        }
    }
}
