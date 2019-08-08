use crate::{entities::Coordinates};

#[derive(Debug)]
pub enum AttackResults {
    Hit,
    Miss
}

#[derive(Debug)]
pub struct Attack {
    coordinates: Coordinates,
    outcome: AttackResults
}