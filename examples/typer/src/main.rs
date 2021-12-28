use eztui::*;
use rand::{thread_rng, Rng};

fn main() {
    KeyTyper::run()
}

#[derive(Debug, Default, Clone)]
struct KeyTyper {
    user: String,
    text: String,
    correct: bool,
    cursor: (u16, u16),
    should_exit: bool,
}

impl Application for KeyTyper {
    fn new() -> Self {
        Self {
            user: String::new(),
            text: String::from("Some random text to type."),
            correct: true,
            cursor: (0, 0),
            should_exit: false,
        }
    }
    fn update(&mut self, input: Keys) -> Command {
        let mut command: Option<Command> = None;
        match input {
            Keys::Key(key) => match key {
                Key::Esc => self.should_exit = true,
                Key::Backspace => {
                    if let Some(_) = self.user.pop() {}
                    command = Some(Command::ClearCurentLine);
                }
                _ => {
                    self.user.push(key.as_char());
                }
            },
            Keys::KeyAndMod { key, .. } => self.user.push(key.as_char()),
        }

        self.correct = self
            .text
            .get(..self.user.len())
            .map(|t| t.contains(&self.user))
            .unwrap_or(false);

        if self.correct && self.text.len() == self.user.len() {
            self.user.clear();
            self.text = generate_new_phrase();
            command = Some(Command::ClearAll);
        }

        match command {
            Some(c) => c,
            None => Command::None,
        }
    }

    fn view(&mut self) -> Element {
        // let user_text = if self.correct {
        //     Text::new(&self.user).with(Color::Green)
        // } else {
        //     Text::new(&self.user).with(Color::Red)
        // };
        let row = Row::new().push(Text::new(&self.text));
        let input = TextInput {
            content: self.user.clone(),
            cursor: (10, 10),
            ..Default::default()
        };
        Column::new().push(row).push(input).into()
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}

fn generate_new_phrase() -> String {
    let s = vec![
        "This is a sentence.",
        "I am on Pop OS that is built on Ubuntu OS.",
        "Where is the coolest state? Yup, TEXAS!!!!",
    ];
    let index = thread_rng().gen_range(0..s.len());
    s[index].to_string()
}
