use crate::models::{Coordinates, Heading};

#[derive(Debug, Copy, Clone)]
pub struct Ship {
    pub origin: Coordinates,
    pub heading: Heading,
    pub length: u16,
}

impl Ship {
    pub fn new(origin: Coordinates, heading: Heading, length: u16) -> Ship {
        Ship {
            origin,
            heading,
            length,
        }
    }

    pub fn get_segment_coordinates(&self) -> Vec<Coordinates> {
        let mut segments: Vec<Coordinates> = vec![];
        for n in 0..self.length {
            match self.heading {
                Heading::East => segments.push(Coordinates {
                    x: self.origin.x + n,
                    y: self.origin.y,
                }),
                Heading::South => segments.push(Coordinates {
                    x: self.origin.x,
                    y: self.origin.y + n,
                }),
            }
        }
        segments
    }

    pub fn default() -> Ship {
        Ship::new(Coordinates::default(), Heading::default(), 2)
    }

    pub fn move_up(self) -> Ship {
        Ship {
            origin: self.origin.get_above(),
            ..self
        }
    }

    pub fn move_right(self) -> Ship {
        Ship {
            origin: self.origin.get_right(),
            ..self
        }
    }

    pub fn move_down(self) -> Ship {
        Ship {
            origin: self.origin.get_below(),
            ..self
        }
    }

    pub fn move_left(self) -> Ship {
        Ship {
            origin: self.origin.get_left(),
            ..self
        }
    }

    pub fn flip(self) -> Ship {
        let heading = match self.heading {
            Heading::South => Heading::East,
            Heading::East => Heading::South,
        };

        Ship { heading, ..self }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_ship() {
        let ship = Ship::new(Coordinates { x: 0, y: 0 }, Heading::East, 2);
        assert_eq!(ship.origin.x, 0);
        assert_eq!(ship.origin.y, 0);
        assert_eq!(ship.heading, Heading::East);
    }

    #[test]
    fn test_get_segment_coordinates() {
        let ship = Ship::new(Coordinates { x: 0, y: 0 }, Heading::East, 2);
        let segments = ship.get_segment_coordinates();
        assert_eq!(segments[0].x, 0);
        assert_eq!(segments[0].y, 0);
        assert_eq!(segments[1].x, 1);
        assert_eq!(segments[1].y, 0);
    }
}
