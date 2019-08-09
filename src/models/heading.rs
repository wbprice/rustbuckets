#[derive(Debug, PartialEq)]
pub enum Heading {
    North,
    East,
    West,
    South
}

impl Default for Heading {
    fn default() -> Self {
        Heading::North
    }
}