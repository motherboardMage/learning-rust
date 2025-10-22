fn main() {
    let s = String::from("Hello, world");

    let h = &s[0..5];
    let w = &s[7..12];

    println!("--{h}--");
    println!("--{w}--");
}
