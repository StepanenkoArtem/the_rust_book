use rand;
use std::cmp::Ordering;
use std::io;

const MIN_RANGE: u32 = 1;
const MAX_RANGE: u32 = 500;

fn main() {
    println!("This is the guess number game!");
    println!("Enter the number from {MIN_RANGE} to {MAX_RANGE}");

    let guessed_number: u32 = get_guess_number(MIN_RANGE, MAX_RANGE);
    let mut counter: u32 = 0;
    let mut is_guessed = false;

    while !is_guessed {
        counter += 1;

        let mut user_input: String = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_number: u32 = user_input.trim().parse().unwrap();

        match user_number.cmp(&guessed_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Bingo!!");
                is_guessed = true;
            }
        }
    }

    println!("You've guessed the number after {counter} attempts");

    let range = 1..=100;

    for element in range {
        println!("{element}");
    }
}

fn get_guess_number(min_range: u32, max_range: u32) -> u32 {
    rand::random_range(min_range..=max_range)
}
