use std::str::FromStr;

use gui;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl gui::Draw for SelectBox {
    fn draw(&self) {
        let _ = 44;
    }
}

fn main() {
    // there's a performance downside to this as method used#
    // has to be resolved at run time (dynamic dispatch)
    let screen = gui::Screen {
        components: vec![
            Box::new(SelectBox {
                width: 10,
                height: 50,
                options: vec![String::from("1 choice")],
            }),
            Box::new(gui::Button {
                width: 10,
                height: 4,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
}
