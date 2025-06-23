struct User {
    name: String,
    email: String,
    age: i8,
}

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
}

fn build_user(age: i8, name: String, email: String) -> User {
    User { name, age, email }
}
