use crate::{
    models::{
        Heading,
        Coordinates
    }
};

#[derive(Debug, Default)]
pub struct Ship {
    pub origin: Coordinates,
    pub heading: Heading,
    pub length: u16
}

impl Ship {
    pub fn new(origin: Coordinates, heading: Heading, length: u16) -> Ship {
        Ship {
            origin,
            heading,
            length
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let ship = Ship::new(
            Coordinates {x: 0, y: 0},
            Heading::East,
            2
        );

        assert_eq!(ship.origin.x, 0);
        assert_eq!(ship.origin.y, 0);
        assert_eq!(ship.heading, Heading::East);
    }
}