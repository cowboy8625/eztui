//TODO: Needs Key access

//! eztui is a crate to make UI in the terminal easier.  It is super early in development but I
//! think eztui will not have a event loop and let the user create it.  Eztui will have
//! Windows: (has a buffer with a width height and a location on screen, can not hold a
//! window),
//! Labels: (Not sure),
//! Groups: (Will hold any strcut that has a Wiget trait.),
//!
use std::io::{Stdout, Write};
use crossterm::{queue, terminal, cursor, style, Result};
use buffy;

/// Base group of methods.
pub trait Wiget {
    fn new(x: u16, y: u16, width: u16, height: u16) -> Self;
    fn set_text(&mut self, location: u16, text: &str);
    fn set_pos(&mut self, x: u16, y: u16);
    fn set_size(&mut self, width: u16, height: u16);
    fn draw(&self, stdout: &mut Stdout);
}

/// Group Widget's together in a Group
pub struct Group {
}

/// Window holds a buffer 
pub struct Window {
    x: u16,
    y: u16,
    buffer: buffy::Buffer,
}

impl Wiget for Window {
    fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x, y, buffer: buffy::Buffer::new(width as usize, height as usize, ' '),
        }
    }

    fn set_text(&mut self, location: u16, text: &str) {
        self.buffer.insert_from_str(location as usize, text);
    }

    fn set_pos(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    /// Sets size of buffer.
    fn set_size(&mut self, width: u16, height: u16) {
        self.buffer.set_size(width as usize, height as usize);
    }

    /// Drawing does not flush.
    fn draw(&self, stdout: &mut Stdout) {
        for (idx, line) in self.buffer.get_lines().iter().enumerate() {
            queue!(
                stdout,
                cursor::MoveTo(self.x, self.y + idx as u16),
                style::Print(line.to_string()),
                ).expect("Draw Error");
        }
    }
}

/// This is just for testing
pub fn clear(stdout: &mut Stdout) {
    queue!(stdout, terminal::Clear(terminal::ClearType::All)).expect("Clear disn't work");
}


