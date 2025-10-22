struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 12,
        height: 5,
    };

    println!("Area of the rectangle is: {}", area(&rect));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
