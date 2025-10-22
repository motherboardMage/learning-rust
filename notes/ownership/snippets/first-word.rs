fn main() {
    let mut s = String::from("Hello world!");

    let first = find_first_word(&s);

    s.clear();

    println!("First word ends at position {first}");
}

fn find_first_word(input: &String) -> usize {
    for (i, ch) in input.chars().enumerate() {
        if ch == ' ' {
            return i;
        }
    }
    input.len()
}
