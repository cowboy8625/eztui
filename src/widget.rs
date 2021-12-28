use crossterm::style::{Color, ContentStyle, StyledContent};
use std::fmt;
pub trait Widget: std::fmt::Display {
    fn fg(&self) -> Option<Color>;
    fn bg(&self) -> Option<Color>;
    fn cursor(&self) -> Option<(u16, u16)> {
        None
    }
    fn draw(&self) -> StyledContent<String> {
        let mut content_style = ContentStyle::new();
        content_style.foreground_color = self.fg();
        content_style.background_color = self.bg();
        StyledContent::new(content_style, self.to_string())
    }
}

impl fmt::Debug for dyn Widget + 'static {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Widget")
            .field("content", &self.to_string())
            .finish()
    }
}
