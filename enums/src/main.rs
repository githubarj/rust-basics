enum  IpAddrKind {
    V6(u8, u8, u8, u8),
    V4(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // when using named fields use curly braces
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call (&self) { // you can call methods on enums as well

    }
}

fn main() {

    let home = IpAddrKind::V4(String::from("::1" ));
    let loopback = IpAddrKind::V6(127 ,0 , 0 , 1);

    let some_number = Some(5);
    let some_char  = Some('e');

    let absent_number : Option<i32> =None;


    println!("Hello, world!");
}
