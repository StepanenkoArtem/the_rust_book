struct User {
    name: String,
    email: String,
    age: i8,
}

struct Color(u8, u8, u8);
struct AlwaysEqual;

fn main() {
    let mut new_user = build_user(12, String::from("Foobar"), String::from("ffobar@gmail.com"));
    new_user.name = String::from("John Doe");
    let another_user = User {
        name: String::from("Foo Bar Bar Booo"),
        ..new_user
    };
    println!(
        "{}, {} - {}",
        another_user.name, another_user.age, another_user.email
    );

    let white = Color(255, 255, 255);
    let black = Color(0, 0, 0);
    example();
    println!("Color code is {}, {}, {}", white.0, white.1, white.2);
    println!("Color code is {}, {}, {}", black.0, black.1, black.2);
}

fn build_user(age: i8, name: String, email: String) -> User {
    User { name, age, email }
}

fn example() -> AlwaysEqual { 
    let example = AlwaysEqual;
    example
}
