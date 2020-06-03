#[derive(Debug)]
enum  IpAddrKind {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //在这里定义方法体
    }
}

fn main() {
    let home: IpAddrKind = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback  : IpAddrKind = IpAddrKind::V6(String::from("::1"));
    println!("{:?}, {:?}", home, loopback);

    let home2: IpAddr = IpAddr::V4(127, 0, 0, 1);
    let loopback2 : IpAddr = IpAddr::V6(String::from("::1"));
    println!("{:?}, {:?}", home2, loopback2);

    let m = Message::Write(String::from("hello"));
    m.call();
}