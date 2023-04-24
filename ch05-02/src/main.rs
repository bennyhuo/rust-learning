use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[derive(PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

impl Rectangle {
    fn double_width(&mut self) {
        self.width *= 2;
    }

    fn area2(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}

impl Drop for Rectangle {
    fn drop(&mut self) {

    }
}

fn main() {
    let mut rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area(),
    );

    dbg!(&rect1);
    println!("rect1 is {rect1:#?}");

    let mut rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(rect1 == rect2);

    dbg!(Rectangle::area2(&rect1));

    dbg!(&rect2);
    rect2.double_width();
    dbg!(&rect2);


    dbg!(rect2.can_hold(&rect1));
    rect1.double_width();
    dbg!(rect2.can_hold(&rect1));
}