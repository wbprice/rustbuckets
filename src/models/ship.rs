use crate::models::{Coordinates, Heading};

#[derive(Debug, Default, Copy, Clone)]
pub struct ShipSegment {
    pub coordinates: Coordinates,
}

impl ShipSegment {
    fn new(coordinates: Coordinates) -> ShipSegment {
        ShipSegment { coordinates }
    }
}

#[derive(Debug)]
pub struct Ship {
    pub origin: Coordinates,
    pub heading: Heading,
    pub length: u16,
    pub segments: Vec<ShipSegment>,
}

impl Ship {
    pub fn new(origin: Coordinates, heading: Heading, length: u16) -> Ship {
        // Create N segments in the heading of the boat.
        let mut segments: Vec<ShipSegment> = vec![];
        for n in 0..length {
            match heading {
                Heading::East => segments.push(ShipSegment::new(Coordinates {
                    x: origin.x + n,
                    y: origin.y,
                })),
                Heading::South => segments.push(ShipSegment::new(Coordinates {
                    x: origin.x,
                    y: origin.y + n,
                })),
            }
        }

        Ship {
            origin,
            heading,
            length,
            segments,
        }
    }

    pub fn default() -> Ship {
        Ship::new(Coordinates::default(), Heading::default(), 2)
    }
}

impl Clone for Ship {
    fn clone(&self) -> Ship {
        Ship {
            origin: self.origin,
            heading: self.heading,
            length: self.length,
            segments: self.segments.clone(),
        }
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
        assert_eq!(ship.segments[0].coordinates.x, 0);
        assert_eq!(ship.segments[0].coordinates.y, 0);
        assert_eq!(ship.segments[1].coordinates.x, 1);
        assert_eq!(ship.segments[1].coordinates.y, 0);
    }

    #[test]
    fn test_new_ship_segment() {
        let ship_segment = ShipSegment::new(Coordinates { x: 0, y: 0 });
        assert_eq!(ship_segment.coordinates.x, 0);
        assert_eq!(ship_segment.coordinates.y, 0);
    }
}
