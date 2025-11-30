fn main() {
    println!("Hello, world!");
    let int: u8 = "215".parse().expect("Value should be number");

    let int = int.checked_add(10).expect("Overflow");
    println!("{int}");

    let float = 4.5;
    let float = float + 4.6;

    println!("{float}");

    let boolean = true;
    println!("{boolean}");

    let point = (45, 50);
    let (x, y) = point; // tuple destructuring
    println!("This is point coordinates: x: {x} and y: {y}");
    println!(
        "This is point coordinates: x: {} and y: {}",
        point.0, point.1
    ); // access by index

    let arr = [1, 2, 3, 4, 5, 6];
    println!("First: {}, second: {}", arr[0], arr[1]);

    let initialized_array = [true; 5];
    println!(
        "Some elements of initialized_array are - first: {}, second: {}",
        initialized_array[0], initialized_array[1]
    );

    let value_assigned_by_if_expression = if true { 5 } else { 7 };
    println!("Value issigned in if-expression is: {value_assigned_by_if_expression}");

    another_function(&8);
    loop_fn(225);

    let months = [
        "January", "February", "March", "April"
    ];


    for_loop(months);

    let unit = (); // unit is Empty tuple
    return unit; // redundant. Unit is returned by default
}

fn another_function(&x: &i32) -> () {
    println!("another_function is called with parameter: {x}");
    // &x
}

fn loop_fn(counter: u8) {
    let mut counter = counter;
    'loop_label: loop {
        counter = match counter.checked_add(1) {
            Some(counter) => {
                println!("Current counter: {counter}");
                counter
            }
            None => {
                println!("Max value hitted");
                break 'loop_label;
            }
        }
    }
}

fn for_loop(collection: [&str; 4]) {
    for element in collection {
        println!("{element}");
    }
}
