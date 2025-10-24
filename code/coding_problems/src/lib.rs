use std::process::exit;

pub fn input_with_exit(prompt: &str) -> String {
    use std::io::{self, Write};

    print!("{prompt}");
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input!");

    if input.trim() == "e" {
        exit(0);
    }
    input
}

pub fn to_number(input: &str) -> Option<u32> {
    match input.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Enter a valid number!");
            None
        }
    }
}

pub fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    for i in 2..=(num as f64).sqrt() as u32 {
        if num % i == 0 {
            return false;
        }
    }
    true
}
