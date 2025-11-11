mod algorithms;

use crate::algorithms::searching::*;
use crate::algorithms::sorting::merge_sort;
use std::io::{self, Write};

fn main() {
    'main: loop {
        let Some(list) = get_numbers_list("\nEnter values or e to exit: ") else {
            break 'main;
        };
        run_inner_menu(list);
    }
}

fn get_numbers_list(prompt: &str) -> Option<Vec<i32>> {
    let mut input = String::new();
    let mut list = Vec::new();

    'outer: loop {
        print!("{prompt}");
        io::stdout().flush().unwrap();

        input.clear();
        list.clear();

        if io::stdin().read_line(&mut input).is_err() {
            println!("Error in reading input, trying again...");
            continue;
        }

        let input = input.trim();

        if input == "e" {
            return None;
        } else if input == "" {
            println!("Enter at least one number!");
            continue;
        }

        for i in input.split_whitespace() {
            match i.parse::<i32>() {
                Ok(num) => list.push(num),
                Err(_) => {
                    println!("Enter valid values!");
                    continue 'outer;
                }
            }
        }
        return Some(list);
    }
}

fn get_number(prompt: &str) -> Option<i32> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();

    match num.trim().parse::<i32>() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Error! Not a number");
            None
        }
    }
}

fn run_inner_menu(list: Vec<i32>) {
    loop {
        let Some(choice) = get_number(
            "\nWhat do you want to do?\n\
               1. Search for a number\n\
               2. Sort the list\n\
               3. Enter a new list\n\
               -> ",
        ) else {
            continue;
        };

        match choice {
            1 => handle_searching(&list),
            2 => handle_sorting(&list),
            3 => break,
            _ => {
                println!("Enter valid value!");
                continue;
            }
        }
    }
}

fn handle_searching(list: &[i32]) {
    let mut tries = 3;
    loop {
        if tries == 0 {
            println!(
                "Incorrect value entered after 3 tries\n\
                 Returning to main menu"
            );
            return;
        }

        let Some(num) = get_number("\nEnter the number to search: ") else {
            tries -= 1;
            continue;
        };

        match linear_search(list, num) {
            Some(idx) => println!("{num} found at index {idx}"),
            None => println!("{num} does not exist in the list"),
        }

        if list.is_sorted() {
            match binary_search(list, num) {
                Some(idx) => println!("{num} found at index {idx}"),
                None => println!("{num} does not exist in the list"),
            }
        } else {
            println!(
                "Cannot perform binary search\n\
                 on an unsorted list"
            );
        }
        break;
    }
}

fn handle_sorting(list: &[i32]) {
    let sorted = merge_sort(list);
    println!("\nSorted list:");
    println!("{:?}", sorted);
}
