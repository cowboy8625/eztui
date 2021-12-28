use super::Widget;
use crossterm::style::Color;
use std::fmt::{self, Display};
#[derive(Debug)]
pub struct Row {
    elements: Vec<Box<dyn Widget>>,
    forground_color: Option<Color>,
    background_color: Option<Color>,
}

impl Row {
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
impl Widget for Row {
    fn fg(&self) -> Option<Color> {
        self.forground_color
    }
    fn bg(&self) -> Option<Color> {
        self.background_color
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.elements
                .iter()
                // TODO: Formatter needs to do spacing
                .map(|w| format!("{} ", w.draw().to_string()))
                .collect::<String>()
        )
    }
}
