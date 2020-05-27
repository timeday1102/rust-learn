fn main() {
    println!("Hello, world!");

    let x: i64 = 257;
    println!("{}!", x);
    another_function(x as i8);

    let x = 127;
    let y = 3.0;
    second_fn(x, y);

    let sum: i32 = {
        let x: i32 = 5;
        let y: i32 = 7;
        x + y
    };
    println!("The sum is: {}!", sum);

    let x: i64 = third_fn(6) as i64;
    println!("the x is: {}!", x);
}

fn another_function(x: i8) {
    println!("another function.");
    println!("The value of x is: {}!", x);
}

fn second_fn(x: i8, y: f64) {
    println!("x is {}!, y is {}!", x, y);
}

fn third_fn(x: i32) -> i32 {
    x + 1
}
