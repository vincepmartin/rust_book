#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle2: &Rectangle) -> bool {
        self.width >= rectangle2.width && self.height >= rectangle2.height
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 30,
    };

    let rectangle2 = Rectangle {
        width: 40,
        height: 40,
    };

    let rectangle3 = Rectangle {
        width: 20,
        height: 20,
    };

    println!(
        "The area of the rectangle is {} square pixel square pixels.",
        rectangle.area()
    );

    // Print out the values of the struct.
    println!("Rectangle is {rectangle:?}");

    // Print out some debug info with line numbers.
    dbg!(&rectangle);

    // Print can rectangle 1 hold rectangle 2 inside of it?
    println!(
        "Can rectangle 1 contain rectangle 2?: {}",
        rectangle.can_hold(&rectangle2)
    );

    // Print can rectangle 1 hold rectangle 3 inside of it?
    println!(
        "Can rectangle 1 contain rectangle 3?: {}",
        rectangle.can_hold(&rectangle3)
    );
}
