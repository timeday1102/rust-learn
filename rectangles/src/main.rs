#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
}

fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width, height)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 60,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );

    println!("rect1 is {:#?}", rect1);
}

fn area1(width: u64, height: u64) -> u64 {
    width * height
}

fn area2(dimensions: (u64, u64)) -> u64 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u64 {
    rectangle.height * rectangle.width
}
