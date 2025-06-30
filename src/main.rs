#[derive(Debug)]
struct Figure {
    width: u32,
    height: u32,
}

impl Figure {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, another_rectangle: &Figure) -> bool {
        another_rectangle.width <= self.width && another_rectangle.height <= self.height
    }

    fn square(width: u32) -> Self {
        Self {
            width,
            height: width,
        }
    }
}

#[derive(Debug)]
enum Message {
    Hello(String),
    Bye(String),
}

fn main() {
    let scale = 2;
    let rectangle = Figure {
        width: 30 * scale,
        height: 34,
    };

    let hello = Message::Hello("Hello".to_string());
    let bye = Message::Bye("Bye".to_string());
    let bigger_rectangle = Figure {
        width: 65,
        height: 35,
    };

    let square = Figure::square(40);
    dbg!(&hello);
    dbg!(square);

    let boolean = true;
    println!("{}", boolean.to_string());
    println!("{}", rectangle.area());

    println!(
        "Is first rectangle can hold then the first one: {}",
        rectangle.can_hold(&bigger_rectangle)
    );

    println!(
        "Is second rectangle can hold then the second one: {}",
        bigger_rectangle.can_hold(&rectangle)
    );
}
