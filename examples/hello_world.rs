use std::io::{stdout, Write};
use eztui::*;

fn main() {
    let mut stdout = stdout();
    let (width, height) = (40, 20);
    let text = "Hello World";
    let mut window = eztui::Window::new(10, 10, width, height);
    window.set_background_color(Color::Rgb{r:0, g:139, b:139});
    window.set_text(width/2-(text.len()as u16)/2, height/2, text, Color::Red, Color::Blue);
    window.draw(&mut stdout);
    stdout.flush().expect("This should flush");
}
