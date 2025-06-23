struct Figure {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Figure {
        width: 114,
        height: 34,
    };

    println!(
        "The are of rectangel with width {} and height {} is {}",
        rectangle.width,
        rectangle.height,
        area(&rectangle)
    )
}

fn area(figure: &Figure) -> u32 {
    figure.width * figure.height
}
