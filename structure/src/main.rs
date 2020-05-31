fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        email: String::from("someone@example.com"),
        acticve: true,
        sign_in_count: 1,
        username: String::from("someuser001"),
    };
    println!("the email of {} is: {}", user1.username, user1.email);

    user1.email = String::from("someone001@example.com");
    println!("the email of {} is: {}", user1.username, user1.email);

    let user2 = build_user(
        String::from("someone002@example.com"),
        String::from("someone002"),
    );
    println!("the email of {} is: {}", user2.username, user2.email);

    let user3 =User {
        email: String::from("someone003@example.com"),
        username: String::from("someone003"),
        ..user1
    };
    println!("the email of {} is: {}", user3.username, user3.email);

    let black = Color(0, 0, 0);
    println!("the value of black is {},{},{}", black.0, black.1, black.2);
}

struct User { // 结构体定义
    username: String,
    email: String,
    sign_in_count: u64,
    acticve: bool,
}

struct Color(i32, i32, i32  );  //元组结构体

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        acticve: true,
        sign_in_count: 1,
    }
}
