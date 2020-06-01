#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
}

impl Rectangle {   // 结构体上的方法
    fn area(&self) -> u64 {  // 一个参数
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {   //多个参数
        self.height > other.height && self.width > other.width
    }

    fn square(size: u64) ->Rectangle  {  //不以self 作为参数的函数
        Rectangle {width:size, height:size}
    }
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

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("Can rect1 hold rect2?  {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?  {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("the area of sq is {} ", sq.area());
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
