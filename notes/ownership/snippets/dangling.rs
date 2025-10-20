fn main() {
    let s = dangle();

    println!("{s}");
}

fn dangle() -> &String {
    let s = String::from("Hello");

    &s;
}
