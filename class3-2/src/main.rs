fn main() {
    let guess: u32 = "42".parse().expect("This is not a number!");
    println!("{}", guess);

    let x = 2.0;
    println!("{}!", x);

    let y: f32 = 3.0;
    println!("{}!", y);

    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 45.0;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;

    // 取余
    let remainder = 45 % 5;

    println!("{}, {}!", sum + product + remainder, difference  + quotient );

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    println!("{}! {}! {}!", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is {}!", y + x as f64 + z as f64);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{}, {}, {}", five_hundred , six_point_four , one);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}!", a[1]);

    let a = [3; 5];
    println!("{}!", a[3]);

    let index = 4;
    println!("{}!", a[index]);
}
