pub enum Level {
    Info,
    Warning,
    Error,
    Success,
}

pub struct Alert {
    pub content: String,
    pub level: Level,
}

impl Alert {
    pub fn new(content: String, level: Level) -> Alert {
        Alert { content, level }
    }
}
