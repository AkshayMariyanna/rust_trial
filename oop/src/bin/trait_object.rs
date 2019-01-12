use oop::{gui::{Button, Draw, Screen}, TextBlock};

struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a SelectBox");
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("MayBe"),
                ],
            }),
            Box::new(Button {
                width: 75,
                height: 10,
                label: String::from("Click Me!"),
            }),
            Box::new(TextBlock {
                width: 75,
                height: 10,
                text: String::from("Some Text"),
            }),
        ],
    };

    screen.run();
}
