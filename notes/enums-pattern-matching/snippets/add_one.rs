fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(value) => Some(value + 1),
    }
}

fn main() {
    let five = Some(5);

    let six = match add_one(five) {
        None => panic!(),
        Some(value) => value,
    };

    println!("{}", six)
}
