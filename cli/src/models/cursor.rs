use crate::models::Coordinates;

#[derive(Debug, Copy, Clone)]
pub struct Cursor {
    pub origin: Coordinates,
}

impl Cursor {
    pub fn default() -> Cursor {
        Cursor {
            origin: Coordinates::default(),
        }
    }

    pub fn move_up(self) -> Cursor {
        Cursor {
            origin: self.origin.get_above(),
            ..self
        }
    }

    pub fn move_right(self) -> Cursor {
        Cursor {
            origin: self.origin.get_right(),
            ..self
        }
    }

    pub fn move_down(self) -> Cursor {
        Cursor {
            origin: self.origin.get_below(),
            ..self
        }
    }

    pub fn move_left(self) -> Cursor {
        Cursor {
            origin: self.origin.get_left(),
            ..self
        }
    }
}
