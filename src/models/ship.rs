use crate::{
    models::{
        Heading,
        Coordinates
    }
};

#[derive(Debug, Default)]
pub struct Ship {
    pub origin: Coordinates,
    pub heading: Heading
}

impl Ship {
    pub fn new(origin: Coordinates, heading: Heading) -> Ship {
        Ship {
            origin,
            heading
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
            Heading::North
        );

        assert_eq!(ship.origin.x, 0);
        assert_eq!(ship.origin.y, 0);
        assert_eq!(ship.heading, Heading::North);
    }
}