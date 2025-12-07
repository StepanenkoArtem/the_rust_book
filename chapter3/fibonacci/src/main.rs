use std::io;

fn main() {
    let mut number: String = String::new();

    println!("Enter the number of Fibonacci element you want to get:");
    match io::stdin().read_line(&mut number) {
        Ok(_) => (),
        Err(_) => println!("Connot read user input "),
    }

    let number: u128 = match number.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            panic!("'{}' is not a number", number.trim());
        }
    };
    let fibo = get_fibonacci_number(number);

    println!("Fibonacci value for number {number} is {fibo}")
}

fn get_fibonacci_number(number: u128) -> u128 {
    let mut temp = [0, 1];
    for _ in 0..=number {
        let old = temp[1];
        let new = temp[0] + temp[1];
        temp = [old, new];
    };
    temp[1]
}
