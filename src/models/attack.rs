use crate::{
    models::{
        Coordinates,
        AttackResult
    }
};

#[derive(Debug)]
pub struct Attack {
    coordinates: Coordinates,
    result: AttackResult
}