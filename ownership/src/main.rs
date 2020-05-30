fn main() {
    println!("Hello, world!");
    owndership();

    let s = String::from("Hello!");
    takes_ownership(s);
    /* error
    println!("{}!", s);
    */

    let s = 5;
    makes_copy(s);
    println!("{}", s);

    let (s, len) = calculate_length(String::from("Hello"));
    println!("{}, {}!", s, len);

    let s = String::from("Hello");
    let len = calculate_length_new(&s);
    println!("{}, {}!", s, len);

    let mut s = String::from("Hello");
    change(&mut s);

    let word = first_word(&s);
    println!("the word is: {}!", word);
}

fn owndership() {
    let s1 = String::from("Hello");
    /* error
    let s2 = s1;
    println!("{}, {}", s1, s2);
    */
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}!", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}!", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn calculate_length(s: String) -> (String, usize) {
    let lenth = s.len();

    (s, lenth)
} 

fn calculate_length_new(s: &String) -> usize { // s 是对 String 的引用
    s.len()
} // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，所以什么也不会发生

//可变引用
fn change(s: &mut String){
    s.push_str(", world!");
    println!("可变引用s: {}!", s);
}

fn first_word(s: &String) -> &str{  //返回 String 参数的一个字节索引值
    let bytes = s.as_bytes(); 
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s
}