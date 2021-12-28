use super::Widget;
use crossterm::style::Color;
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub struct Text {
    content: String,
    forground_color: Option<Color>,
    background_color: Option<Color>,
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            content: text.to_string(),
            forground_color: None,
            background_color: None,
        }
    }

    pub fn with(mut self, color: Color) -> Self {
        self.forground_color = Some(color);
        self
    }
    pub fn on(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }
}

impl Widget for Text {
    fn fg(&self) -> Option<Color> {
        self.forground_color
    }
    fn bg(&self) -> Option<Color> {
        self.background_color
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
