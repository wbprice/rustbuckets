#[derive(Debug, PartialEq)]
pub enum Heading {
    East,
    South
}

impl Default for Heading {
    fn default() -> Self {
        Heading::East
    }
}