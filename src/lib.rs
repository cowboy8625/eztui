//TODO: Needs Key access
//FIXME: Adding Color to the Cell's isnt working,   I may need to strip the ascii code or i may
//need to Get background codes and not forground.  SetForegroundColor
//
//
//On further inspection, i have seen that the SetBackgroundColor returns a color text reset so when
//i show my text its like SetBackgroundColor -> color text reset my text
//so it never shows the color, easy.

//! eztui is a crate to make UI in the terminal easier.  It is super early in development but I
//! think eztui will not have a event loop and let the user create it.  Eztui will have
//! Windows: (has a buffer with a width height and a location on screen, can not hold a
//! window),
//! Labels: (Not sure),
//! Groups: (Will hold any strcut that has a Wiget trait.),
//!
pub use buffy::Color;
use std::io::{Stdout, Write};
use crossterm::{queue, terminal, cursor, style};
use buffy::Buffer;

/// Base group of methods.
pub trait Wiget {
    fn new(x: u16, y: u16, width: u16, height: u16) -> Self;
    fn set_text(&mut self, x: u16, y: u16, text: &str, fg: buffy::Color, bg: buffy::Color);
    fn set_background_color(&mut self, color: Color);
    fn set_pos(&mut self, x: u16, y: u16);
    fn set_size(&mut self, width: u16, height: u16);
    fn draw(&mut self, stdout: &mut Stdout);
}

/// Group Widget's together in a Group
pub struct Group {
}

/// Window holds a buffer 
pub struct Window {
    x: u16,
    y: u16,
    buffer: Buffer,
}

impl Wiget for Window {
    fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x, y, buffer: buffy::Buffer::new(width as usize, height as usize, ' ', buffy::Color::White, buffy::Color::Black),
        }
    }

    fn set_text(&mut self, x: u16, y: u16, text: &str, fg: buffy::Color, bg: buffy::Color) {
        self.buffer.replace_line(x as usize, y as usize, text, fg, bg);
    }

    fn set_background_color(&mut self, color: Color) {
        self.buffer.color.iter_mut().for_each(|(_, b)| *b = color);
    }

    fn set_pos(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    /// Sets size of buffer.
    fn set_size(&mut self, width: u16, height: u16) {
        self.buffer.width = width as usize;
        self.buffer.height = height as usize;
    }

    /// Drawing does not flush.
    fn draw(&mut self, stdout: &mut Stdout) {
        let (x, y) = (self.x, self.y);
        for q in self.buffer.queue.drain(..) {
            for (i, c) in q.cells.iter().enumerate() {
                queue!(
                    stdout,
                    cursor::MoveTo(x + q.x + i as u16, y + q.y),
                    style::SetColors(style::Colors::new(q.color[i].0.into(), q.color[i].1.into())),
                    style::Print(c)
                ).expect("Draw Error");
            }
        }
    }
}

/// This is just for testing
pub fn clear(stdout: &mut Stdout) {
    queue!(stdout, terminal::Clear(terminal::ClearType::All)).expect("Clear disn't work");
}


