use std::io;

fn main() {
    let mut number: String = String::new();

    println!("Enter the number of Fibonacci element you want to get:");
    match io::stdin().read_line(&mut number) {
        Ok(_) => (),
        Err(_) => println!("Connot read user input "),
    }

    let number: i128 = match number.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            panic!("'{}' is not a number", number.trim());
        }
    };
    let fibo = get_fibonacci_number(number);

    println!("Fibonacci value for number {number} is {fibo}")
}

fn get_fibonacci_number(number: i128) -> i128 {
    if number == 1 {
        return number;
    }

    match number.checked_mul(number - 1) {
        Some(value) => get_fibonacci_number(value),
        None => {
            panic!("The result is too high to be calculated")
        }
    }
}
