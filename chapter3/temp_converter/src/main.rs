use std::io;

fn main() {
    let mut user_input: String = String::new();

    println!("Enter temparature in Fahrenheit: ");
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => (),
        Err(_) => println!("Incorrect input"),
    }
    let fahrenheit: f32 = match user_input.trim().parse() {
        Ok(temperature) => temperature,
        Err(_) => {
            println!("Your input cannot be treated as temparature");
            0.0
        }
    };

    let celcius: f32 = (fahrenheit - 32.0) * (5.0 / 9.0);
    println!("{fahrenheit} degrees in Fahrenheit is eq {celcius} degrees in celcius")
}
