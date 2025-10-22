fn main() {
    let mut s = String::from("Hello world!");

    let first = find_first_word(&s);

    println!("First word was {first}");

    s.clear();
}

fn find_first_word(input: &str) -> &str {
    for (i, ch) in input.chars().enumerate() {
        if ch == ' ' {
            return &input[..i];
        }
    }
    &input[..]
}
