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