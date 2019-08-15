#[derive(Default, Debug, Copy, Clone)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Coordinates {
    pub fn move_up(self) -> Coordinates {
        Coordinates {
            y: self.y - 1,
            ..self
        }
    }

    pub fn move_right(self) -> Coordinates {
        Coordinates {
            x: self.x + 1,
            ..self
        }
    }

    pub fn move_down(self) -> Coordinates {
        Coordinates {
            y: self.y + 1,
            ..self
        }
    }

    pub fn move_left(self) -> Coordinates {
        Coordinates {
            x: self.x - 1,
            ..self
        }
    }
}
