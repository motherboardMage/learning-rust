use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input!");

    let first = find_first_word(&input);
    println!("First word was: {}", first);
}

fn find_first_word(input: &String) -> &str {
    for (i, ch) in input.chars().enumerate() {
        if ch == ' ' {
            return &input[0..i];
        }
    }
    &input[..]
}
