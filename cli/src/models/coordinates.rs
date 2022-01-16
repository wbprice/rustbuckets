#[derive(Default, Debug, Copy, Clone)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Coordinates {
    pub fn get_above(self) -> Coordinates {
        Coordinates {
            y: self.y - 1,
            ..self
        }
    }

    pub fn get_right(self) -> Coordinates {
        Coordinates {
            x: self.x + 1,
            ..self
        }
    }

    pub fn get_below(self) -> Coordinates {
        Coordinates {
            y: self.y + 1,
            ..self
        }
    }

    pub fn get_left(self) -> Coordinates {
        Coordinates {
            x: self.x - 1,
            ..self
        }
    }
}
