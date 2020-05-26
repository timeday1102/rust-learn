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

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
    println!("{}!", heart_eyed_cat);
    
}
