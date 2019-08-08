use crate::{entities::Coordinates};

pub enum AttackResults {
    Hit,
    Miss
}

pub struct Attack {
    coordinates: Coordinates,
    outcome: AttackResults
}