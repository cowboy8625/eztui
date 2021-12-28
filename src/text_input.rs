use super::{Color, Widget};
use std::fmt::{self, Display};
#[derive(Debug, Default)]
pub struct TextInput {
    pub content: String,
    pub cursor: (u16, u16),
    pub foregound_color: Option<Color>,
    pub background_color: Option<Color>,
}

impl TextInput {}

impl Widget for TextInput {
    fn fg(&self) -> Option<Color> {
        None
    }
    fn bg(&self) -> Option<Color> {
        None
    }
    fn cursor(&self) -> Option<(u16, u16)> {
        Some(self.cursor)
    }
}

impl Display for TextInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
