#[derive(Debug)]
struct Figure {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 3;
    let rectangle = Figure {
        width: 30 * scale,
        height: 34,
    };

    println!(
        "The are of rectangel with width {} and height {} is {}",
        rectangle.width,
        rectangle.height,
        area(&rectangle)
    );
    dbg!(&rectangle);
}

fn area(figure: &Figure) -> u32 {
    figure.width * figure.height
}
