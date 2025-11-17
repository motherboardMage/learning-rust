use std::io::{self, Write};

fn binary_guess(range: u32) {
    let mut start = 0;
    let mut end = range;
    let mut iters = 0;

    while start < end {
        let guess = start + (end - start) / 2;
        iters += 1;

        let mut comparison = String::new();
        println!("Is {guess} bigger or smaller or equal to your number?");
        io::stdin().read_line(&mut comparison).unwrap();

        match comparison.trim() {
            "b" => end = guess - 1,
            "s" => start = guess + 1,
            "e" => {
                println!("Found your number in {iters} steps!");
                return;
            }
            _ => println!("Enter valid input!"),
        }
    }
}

fn main() {
    let mut range = String::new();

    print!("Enter range: ");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut range);

    let range: u32 = range.trim().parse().unwrap();

    binary_guess(range);
}
