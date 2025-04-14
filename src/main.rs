use rand;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("This is the guess number game!");
    println!("Enter the number from 0 to 100");

    let mut user_input: String = String::new();
    let guessed_number: u32 = rand::random_range(0..=100);

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let user_number: u32 = user_input.trim().parse().unwrap();

    match user_number.cmp(&guessed_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("Bingo!!"),
    }
}
