#[derive(Debug, Default)]
pub struct Board {
    pub width: u16,
    pub height: u16,
}

impl Board {
    pub fn new(width: u16, height: u16) -> Board {
        Board { width, height }
    }
}
