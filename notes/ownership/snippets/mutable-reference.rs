fn main() {
    let mut s = String::from("Hello");

    add_world(&mut s);

    println!("{s}");
}

fn add_world(input: &mut String) {
    input.push_str(", world!");
}
