use super::{Column, Row, Text, Widget};
use crossterm::style::Color;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Element {
    pub elements: Vec<Box<dyn Widget>>,
}

impl Widget for Element {
    fn fg(&self) -> Option<Color> {
        None
    }
    fn bg(&self) -> Option<Color> {
        None
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.elements
                .iter()
                .map(|w| w.to_string())
                .collect::<String>()
        )
    }
}

impl From<Text> for Element {
    fn from(text: Text) -> Self {
        Self {
            elements: vec![Box::new(text)],
        }
    }
}

impl From<Row> for Element {
    fn from(row: Row) -> Self {
        Self {
            elements: vec![Box::new(row)],
        }
    }
}

impl From<Column> for Element {
    fn from(column: Column) -> Self {
        Self {
            elements: vec![Box::new(column)],
        }
    }
}
