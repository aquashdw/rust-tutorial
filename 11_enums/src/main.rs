use std::net::Ipv4Addr;
use std::ptr::null;

enum IpAddrKind {
    V4,
    V6,
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// instead of enum inside a struct, we can add value inside an enum
enum IpAddr {
    // V4(String),
    // V6(String),
    // each enums can have different types and amounts of data
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    //
    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // using enums with values
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    // ...this ip address example is in the standard library by the way.
    // let ipv4_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    message_enum_example();
    option_enum_example();
}

// this enum has four variants
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// it could be similar to creating 4 different structs,
// while creating structs means creating different types.
/*
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
 */
// in contrast to enum, which all fall under the same `Message` type.

// ...and we can create impl blocks for enums, making methods just like structs.
impl Message {
    fn call(&self) {
        // TODO: message body
    }
}

fn message_enum_example() {
    let message = Message::Write(String::from("hello"));
    message.call();
}

fn option_enum_example() {
    // `Option` is an enum in the standard library.
    // it represents a value could be something or nothing.
    // it looks like the following:
    /*
    enum Option<T> {
        None,
        Some(T),
    }
     */
    // ...which is similar to Java Optional<T>.

    // `T` can be deduced with `Some`
    let some_number = Some(5);
    let some_char = Some('e');
    // while it can't be deduced with `None`
    let absent_number : Option<i32> = None;

    // they have `is_some` and `is_none` methods to check if value exists
    let result = if some_number.is_some() && absent_number.is_some() {
        some_number.unwrap() + absent_number.unwrap()
    } else { -1 };
    println!("{result}");
}
