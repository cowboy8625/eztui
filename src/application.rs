use crossterm::{cursor, event, execute, queue, style, terminal};
use std::io::{stdout, Write};

use super::{Command, Element, Keys};
pub trait Application: Sized {
    fn new() -> Self;
    fn update(&mut self, input: Keys) -> Command;
    fn view(&mut self) -> Element;
    fn should_exit(&self) -> bool {
        false
    }
    fn run()
    where
        Self: 'static,
    {
        run::<Instance<Self>>();
    }
}

struct Instance<A>(A)
where
    A: Application;

impl<A: Application> Drop for Instance<A> {
    fn drop(&mut self) {
        execute!(
            stdout(),
            terminal::LeaveAlternateScreen,
            cursor::RestorePosition,
            cursor::Show,
        )
        .expect("Failed to Exit Alt Screen");
        terminal::disable_raw_mode().expect("Failed to Raw Mode");
    }
}
impl<A> runtime::terminal::Program for Instance<A>
where
    A: Application,
{
    fn update(&mut self, input: Keys) -> Command {
        self.0.update(input)
    }
    fn view(&mut self) -> Element {
        self.0.view()
    }
}

impl<A> runtime::Application for Instance<A>
where
    A: Application,
{
    fn new() -> Self {
        Self(A::new())
    }

    fn should_exit(&self) -> bool {
        self.0.should_exit()
    }
}

pub fn run<A>()
where
    A: runtime::Application,
{
    let mut app = A::new();
    let mut is_running = app.should_exit();
    let mut writer = stdout();
    execute!(
        &mut writer,
        cursor::SavePosition,
        terminal::EnterAlternateScreen,
    )
    .expect("Failed to enter into alternate Screen.");

    terminal::enable_raw_mode().expect("Failed to enter into raw mode.");

    let (_width, _height) = terminal::size().expect("Failed to get terminal size.");
    let views = app.view();
    let mut cursor: Option<(u16, u16)> = None;
    for (i, view) in views.elements.iter().enumerate() {
        queue!(&mut writer, cursor::MoveTo(0, i as u16), style::Print(view),)
            .expect("Failed to Print View.");
        if view.cursor().is_some() {
            is_running = true;
            cursor = view.cursor();
        }
    }
    if let Some((x, y)) = cursor {
        queue!(&mut writer, cursor::MoveTo(x, y),).expect("Failed to Move Cursor.");
    }

    writer.flush().expect("Failed to flush view.");

    while !is_running {
        let mut cursor: Option<(u16, u16)> = None;
        if event::poll(std::time::Duration::from_millis(50)).expect("Failed to Poll") {
            let events = event::read().expect("Failed to Read Event");
            match events {
                event::Event::Key(keyevent) => match app.update(Keys::from(keyevent)) {
                    Command::ClearCurentLine => {
                        queue!(
                            &mut writer,
                            terminal::Clear(terminal::ClearType::CurrentLine),
                        )
                        .expect("Failed to Clear CurrentLine");
                    }
                    Command::ClearAll => {
                        queue!(&mut writer, terminal::Clear(terminal::ClearType::All),)
                            .expect("Failed to Clear CurrentLine");
                    }
                    Command::None => {}
                },
                _ => {}
            }

            let views = app.view();
            for (i, view) in views.elements.iter().enumerate() {
                queue!(&mut writer, cursor::MoveTo(0, i as u16), style::Print(view),)
                    .expect("Failed to do loop Print View.");
                if view.cursor().is_some() {
                    cursor = view.cursor();
                }
            }
            if let Some((x, y)) = cursor {
                queue!(&mut writer, cursor::MoveTo(x, y), cursor::Show,)
                    .expect("Failed to Move Cursor.");
            }

            writer.flush().expect("Failed to Flush loop view.");
        }
        is_running = app.should_exit();
        if cursor.is_some() {
            is_running = true;
        }
    }
}

pub(crate) mod runtime {
    use terminal::Program;
    pub trait Application: Program {
        fn new() -> Self;
        fn should_exit(&self) -> bool;
    }

    pub mod terminal {
        use crate::Command;
        use crate::Element;
        use crate::Keys;
        pub trait Program {
            fn update(&mut self, input: Keys) -> Command;
            fn view(&mut self) -> Element;
        }
    }
}
