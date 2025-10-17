fn main() {
    let s = gives_ownership();

    println!("{s}");
}

fn gives_ownership() -> String {
    let some_string = String::from("Owned by s");

    some_string
}
