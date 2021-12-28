use eztui::{Application, Color /*,Settings, Result*/, Command, Element, Key, Keys, Text};

// #[derive(Debug)]
// enum Message {
//     NoOp,
// }

struct App {
    is_running: bool,
    keys_pressed: String,
}

impl Application for App {
    // type Message = Message;

    fn new() -> Self {
        Self {
            is_running: false,
            keys_pressed: String::new(),
        }
    }
    fn update(&mut self, keys: Keys) -> Command {
        match keys {
            Keys::Key(key) => {
                match key {
                    Key::Esc => self.is_running = true,
                    _ => {}
                }
                self.keys_pressed = format!("KEY: {:?}", key);
                return Command::ClearCurentLine;
            }
            Keys::KeyAndMod { key, modk } => {
                match (key, modk) {
                    (Key::LC, Key::Ctrl) => self.is_running = true,
                    _ => {}
                }
                self.keys_pressed = format!("KEY: {:?} with Mod Key: {:?}", key, modk);
                return Command::ClearCurentLine;
            }
        }
    }
    fn view(&mut self) -> Element /*<'_, Self::Message>*/ {
        Text::new(&self.keys_pressed).with(Color::Cyan).into()
    }

    fn should_exit(&self) -> bool {
        self.is_running
    }
}

fn main() /*-> Result */
{
    App::run(/*Settings::default()*/)
}
