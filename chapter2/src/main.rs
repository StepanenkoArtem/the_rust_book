use rand::Rng;
use std::{cmp::Ordering, io};

const INVITATION: &str = "Guess the number";

fn main() {
    println!("{INVITATION}");
    println!("Input your guess:");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };
        let number: u8 = rand::rng().random_range(1..=10);

        match guess.cmp(&number) {
            Ordering::Less => println!("Value too small"),
            Ordering::Greater => println!("Value too high"),
            Ordering::Equal => {
                println!("YOU WIN!!!");
                break;
            }
        }
        println!("You guess {guess} but correct answer is {number}");
    }
}
