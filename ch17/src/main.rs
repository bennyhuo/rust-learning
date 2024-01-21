fn main() {
    println!("Hello, world!");

    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 10,
                height: 5,
                label: String::from("Hello"),
            }),
            Box::new(SelectBox {
                width: 10,
                height: 5,
                options: vec![],
            }),
            Box::new(String::from("bennyhuo")),
        ],
    };

    screen.run();
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct View {
    width: u32,
    height: u32,
}

pub struct Button {
    view: View,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("Draw Button.")
    }
}

pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for crate::SelectBox {
    fn draw(&self) {
        // code to actually draw a button
        println!("Draw SelectBox.")
    }
}

impl Draw for String {
    fn draw(&self) {
        println!("Hello !! {self}")
    }
}
