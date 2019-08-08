use entities::{Coordinates}
use systems::{Faction}

#[derive(Debug, Default)]
struct Board {
    origin: Coordinates,
    width: u16,
    height: u16,
    faction: Faction
}