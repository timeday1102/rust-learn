fn main() {
    let number = 7;
    if number == 0 {
        println!("number is equal then 0!");
    } else if number > 0 {
        println!("number is great then 0!");
    } else {
        println!("number is less then 0!");
    }

    let condition: bool = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The number is {}!", number);
    
}
