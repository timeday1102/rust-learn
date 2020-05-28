fn main() {
    let mut counter = 0;

    let mut result =  
    loop {
        counter = counter + 1;
        if counter == 10 {
            break counter * 2
        }
    };
    println!("The result is {}!", result);
    println!("LIFTOFF!!!");

    while result != 0 {
        result = result - 1;
    }
    println!("The result is {}!", result);
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        print!("{}!", a[index]);
        index = index + 1;
    }
    println!("The index is {}!", index);
    println!("LIFTOFF!!!");

    for element in a.iter() {
        println!("The value of element is {}!", element);
    }
    println!("LIFTOFF!!!");

    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");

    for number in 1..=4 {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");

    for number in (1..=5).rev() {
        println!("{}", number);
    };
    println!("LIFTOFF!!!");
}


