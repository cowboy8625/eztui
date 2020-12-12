use std::io::{stdout, Write};
use eztui::*;

fn main() {
    let mut stdout = stdout();
    let mut window = eztui::Window::new(0, 0, 10, 10);
    window.set_text(0, "Hello World\nNew lines also work like normal");
    window.set_pos(50, 10);
    // FIXME: Remove clear some how.
    eztui::clear(&mut stdout);
    window.draw(&mut stdout);
    stdout.flush().expect("This should flush");
}
