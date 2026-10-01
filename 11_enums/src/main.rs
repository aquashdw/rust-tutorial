use std::net::Ipv4Addr;
use std::ptr::null;
use crate::UsState::{Alabama, Alaska};

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
    match_flow_example();
    match_option_example();
    catch_all_example();
    if_let_else_example();
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

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // omitted
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // match (expression) {}
    // execute or evaluate the code that matches the expression
    match coin {
        // this is called a match arm
        Coin::Penny => {  // `=>` separates the pattern and code.
            println!("Lucky Penny!");
            1
        }  // when we use curly brackets commas are optional
        // normally each arm is separated with commas
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // when using enums with values, we can use variables to bind the values
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn match_flow_example() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(Alaska));
    let dime_value = value_in_cents(Coin::Dime);
    println!("{dime_value}");
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn match_option_example() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}


fn catch_all_example() {
    let dice_roll = 9;
    // matches are exhaustive; all possible cases must be met
    match dice_roll {
        3 => add_fancy_hat(),
        7 => rm_fancy_hat(),
        // for all other possible values, we can bind a variable
        other => move_player(other),
        // if we don't need the value, we use underscore
        _ => println!("reroll"),
        // if we don't want anything to happen at all, we use empty tuples
        _ => ()
    }
}

fn add_fancy_hat() {
    println!("fancy hat added");
}

fn rm_fancy_hat() {
    println!("fancy hat removed");
}

fn move_player(num_spaces: u8) {
    println!("player moves: {num_spaces} spaces");
}

fn if_let_else_example() {
    // say we want to execute code only when a max value is configured
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("Configured max: {max}"),
        // because `match`s are exhaustive, this arm is necessary, and kind of redundant
        _ => (),
    }

    // we can use `if let` for this case
    if let Some(max) = config_max {  // we can assign `config_max`'s value to `max`
        println!("Configured max: {max}")
    }
    // so `if let`s are like syntax sugar for `match` with one arm.


    // we can add a `else` to the whole occasion
    let mut count = 0;

    // for situations that doesn't require the value,
    // let coin = Coin::Quarter(Alabama);
    let coin = Coin::Dime;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}"),
        _ => count += 1,
    }

    // we can use the else block
    let coin = Coin::Quarter(Alaska);
    if let Coin::Quarter(state ) = coin {
        println!("State quarter from {state:?}")
    } else {
        count += 1;
    }

    println!("None quarter count: {count}");

    describe_state_quarter(Coin::Quarter(Alabama));
    describe_state_quarter(Coin::Penny);
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            _ => false,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    // instead of if let...else,
    // we can just use let ... else to either assign. or run code.
    let Coin::Quarter(state) = coin else { return None; };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
