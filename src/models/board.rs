use crate::{
    models::{
    Coordinates,
    Faction
}};

#[derive(Debug, Default)]
pub struct Board {
    origin: Coordinates,
    width: u16,
    height: u16,
    faction: Faction
}