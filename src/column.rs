use super::Widget;
use crossterm::style::Color;
use std::fmt::{self, Display};
#[derive(Debug)]
pub struct Column {
    elements: Vec<Box<dyn Widget>>,
    forground_color: Option<Color>,
    background_color: Option<Color>,
}
impl Column {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
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

    pub fn push(mut self, widget: impl Widget + 'static) -> Self {
        self.elements.push(Box::new(widget));
        self
    }
}
impl Widget for Column {
    fn fg(&self) -> Option<Color> {
        self.forground_color
    }
    fn bg(&self) -> Option<Color> {
        self.background_color
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.elements
                .iter()
                .map(|w| format!("{}\r\n", w.draw().to_string()))
                .collect::<String>()
        )
    }
}
