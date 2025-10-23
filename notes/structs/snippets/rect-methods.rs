struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 15,
        height: 5,
    };

    let rect1 = Rectangle {
        width: 16,
        height: 7,
    };

    println!("The area of the rectangle is: {}", rect.area());
    println!("Width of the rectangle is {}", rect.width());

    if rect.can_hold(&rect1) {
        println!("rect can hold rect1");
    } else {
        println!("rect cannot hold rect1");
    }
}
