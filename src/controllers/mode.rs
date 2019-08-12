#[derive(Debug, PartialEq)]
pub enum Mode {
    Title,
    Setup,
    Play,
    Endscreen,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Title
    }
}
