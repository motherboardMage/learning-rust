mod gemini;

use gemini::{Config, Solver};
use std::io::{self, Write};
use std::process;

fn main() {
    let config = match Config::parse() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            println!("\nUsage: perfect_match [-d DIGITS] [-c CHARSET]");
            println!("Options:");
            println!("  -d, --digits  Number of characters in the code (1-8). Default: 4.");
            println!("  -c, --chars   Character set ('numeric' or 'alphanumeric'). Default: numeric.");
            process::exit(1);
        }
    };

    let mut solver = match Solver::new(&config) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error initializing solver: {}", e);
            process::exit(1);
        }
    };

    println!("\n--- Welcome to Perfect Match ---");
    println!("An optimal solver for the code guessing game.");
    println!("\nGame Configuration:");
    println!("  Length: {} characters", config.digits());
    println!("  Charset: {}", config.char_set().name());
    println!("\nCommands:");
    println!("  'me'        -> Get the optimal code for you to guess.");
    println!("  'other'     -> Input another player's guess and the result.");
    println!("  'status'    -> Show the number of remaining possible candidates.");
    println!("  'config'    -> Show the current game configuration.");
    println!("  'quit'      -> Exit the program.");

    let stdin = io::stdin();
    let mut input_buffer = String::new();

    loop {
        if solver.candidates().len() == 1 {
            println!(
                "\n[!!!] SOLVED! The code is: {}",
                solver.vec_to_string(solver.candidates().first().unwrap())
            );
            break;
        }
        if solver.candidates().is_empty() {
            println!("\n[!!!] Error: No codes fit the constraints. Someone gave wrong feedback.");
            break;
        }

        print!("\n(perfect_match)> ");
        io::stdout().flush().unwrap();

        input_buffer.clear();
        stdin.read_line(&mut input_buffer).expect("Failed to read line");
        let cmd = input_buffer.trim().to_lowercase();

        match cmd.as_str() {
            "quit" | "exit" => {
                println!("Goodbye!");
                break;
            }
            "config" => {
                println!("\nGame Configuration:");
                println!("  Length: {} characters", config.digits());
                println!("  Charset: {}", config.char_set().name());
            }
            "status" => {
                let remaining = solver.candidates().len();
                let total = solver.total_search_space();
                let percentage = (remaining as f64 / total as f64) * 100.0;
                println!("[*] Candidates remaining: {} / {} ({:.2}%)", remaining, total, percentage);

                if remaining < 15 {
                    let examples: Vec<String> = solver.candidates().iter().map(|c| solver.vec_to_string(c)).collect();
                    println!("[*] Possibilities: {:?}", examples);
                }
            }
            "me" => {
                if let Some(guess_idx) = solver.get_best_guess_idx() {
                    let guess = &solver.candidates()[guess_idx];
                    let guess_str = solver.vec_to_string(guess);
                    println!("---------------------------------");
                    println!(" >>> YOUR BEST GUESS IS: {}", guess_str);
                    println!("---------------------------------");

                    loop {
                        print!("How many characters matched? (0-{}): ", config.digits());
                        io::stdout().flush().unwrap();
                        input_buffer.clear();
                        stdin.read_line(&mut input_buffer).unwrap();

                        match input_buffer.trim().parse::<usize>() {
                            Ok(matches) if matches <= config.digits() => {
                                let guess_clone = guess.clone();
                                solver.add_guess_to_history(&guess_clone, matches);
                                solver.prune(&guess_clone, matches);
                                break;
                            }
                            _ => println!("[!] Please enter a valid number between 0 and {}.", config.digits()),
                        }
                    }
                } else {
                    println!("[!] Could not determine a guess. No candidates remaining.");
                }
            }
            "other" => {
                let guess_vec: Vec<u8>;
                loop {
                    print!("What code did they guess? ");
                    io::stdout().flush().unwrap();
                    input_buffer.clear();
                    stdin.read_line(&mut input_buffer).unwrap();
                    let g_str = input_buffer.trim();

                    if g_str.len() == config.digits() {
                        if let Some(v) = solver.string_to_vec(g_str) {
                            guess_vec = v;
                            break;
                        }
                    }
                    println!("[!] Invalid format. Must be {} characters using {}.", config.digits(), config.char_set().name());
                }

                loop {
                    print!("How many matches for {}? ", solver.vec_to_string(&guess_vec));
                    io::stdout().flush().unwrap();
                    input_buffer.clear();
                    stdin.read_line(&mut input_buffer).unwrap();

                    match input_buffer.trim().parse::<usize>() {
                        Ok(matches) if matches <= config.digits() => {
                            solver.add_guess_to_history(&guess_vec, matches);
                            solver.prune(&guess_vec, matches);
                            break;
                        }
                        _ => println!("[!] Please enter a valid number between 0 and {}.", config.digits()),
                    }
                }
            }
            _ => println!("[!] Unknown command. Type 'me', 'other', 'status', 'config', or 'quit'."),
        }
    }
}
