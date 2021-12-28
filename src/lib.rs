mod application;
mod column;
mod command;
mod element;
mod key;
mod row;
mod text;
mod text_input;
mod widget;
pub use application::Application;
pub use column::Column;
pub use command::Command;
pub use crossterm::style::Color;
pub use element::Element;
pub use key::{Key, Keys};
pub use row::Row;
pub use text::Text;
pub use text_input::TextInput;
pub use widget::Widget;

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn application_trait() {
    //     struct App;
    //     impl Application for App {
    //         fn update(&mut self) {}
    //         fn view(&mut self) -> Element {
    //             let row1 = Row::new()
    //                 .push(Text::new("Row 1 Item 1"))
    //                 .push(Text::new("Row 1 Item 2"));
    //             let row2 = Row::new()
    //                 .push(Text::new("Row 2 Item 1"))
    //                 .push(Text::new("Row 2 Item 2"));
    //             let column = Column::new().push(row1).push(row2);
    //             column.into()
    //         }
    //     }
    // }
    #[test]
    fn widget_to_element() {
        let text: Element = Text::new("Text").into();
        eprintln!("{:?}", text);
        eprintln!("^ TEXT ^");

        // let row: Element = Row::new().into();
        // eprintln!("{:?}", row);
        // eprintln!("^ EMPTY ROW ^");
        //
        // let column: Element = Column::new().into();
        // eprintln!("{:?}", column);
        // eprintln!("^ EMPTY COLUMN ^");
        //
        // let row = Row::new().push(Text::new("Text in Row"));
        // eprintln!("{:?}", Element::from(row));
        // eprintln!("^ TEXT IN ROW ^");
        //
        // let column = Column::new().push(Text::new("Text in Column"));
        // eprintln!("{:?}", Element::from(column));
        // eprintln!("^ TEXT IN COLUMN ^");
        //
        // let row1 = Row::new()
        //     .push(Text::new("Row 1 Item 1"))
        //     .push(Text::new("Row 1 Item 2"));
        // let row2 = Row::new()
        //     .push(Text::new("Row 2 Item 1"))
        //     .push(Text::new("Row 2 Item 2"));
        // let column = Column::new().push(row1).push(row2);
        // eprintln!("{:?}", Element::from(column));
        // eprintln!("^ ROW IN TEXT IN COLUMN X2 ^");
        assert!(false);
    }
}
