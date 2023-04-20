#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // pass in self as a parameter
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    if rectangle.can_hold(&Rectangle {
        width: 10,
        height: 40,
    }) {
        println!("The rectangle can hold the other rectangle.");
    } else {
        return println!("The rectangle cannot hold the other rectangle.");
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );
}
