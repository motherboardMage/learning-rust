use coding_problems::{input_with_exit, to_number};

fn main() {
    'main_loop: loop {
        let num_str = input_with_exit("Enter a number or 'e' to exit: ");

        let num = match to_number(&num_str) {
            Some(n) => n,
            None => continue,
        };

        let len = num_str.trim().len();
        let mut sum: u32 = 0;

        for ch in num.to_string().chars() {
            match ch.to_digit(10) {
                Some(digit) => sum += digit.pow(len as u32),
                None => {
                    println!("Enter a valid number!");
                    continue 'main_loop;
                }
            }
        }

        if sum == num {
            println!("{num} is an Armstrong number!");
        } else {
            println!("{num} is not an Armstrong number!");
        }
    }
}
