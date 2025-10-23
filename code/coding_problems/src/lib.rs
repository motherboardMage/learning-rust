pub fn get_number_from_user(prompt: &str) -> i64 {
    use std::io::{self, Write};

    loop {
        print!("{prompt}");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read input!");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Enter a valid number!");
                continue;
            }
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
