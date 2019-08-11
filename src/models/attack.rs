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
    result: AttackResult
}

impl Attack {
    fn new(self, ships: &Vec<Ship>, coordinates: &Coordinates) {
        
    }
}