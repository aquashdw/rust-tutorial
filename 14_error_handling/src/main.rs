use std::fs::File;
use std::{fs, io};
use std::io::{ErrorKind, Read};

fn main() {
    // panicking();
    using_result();
}


fn panicking() {
    // `panic!` is for unrecoverable errors
    // panic!("burn the house down");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let index: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            // panicking here because index is required for this function
            panic!("Please type a integer!");
        },
    };


    let v = vec![1, 2, 3];
    // this may panic if index is out of bounds
    v[index];
}

fn using_result() {
    // `Result<T>` is an enum
    let filename = String::from("hello.txt");
    let greeting_file_result = File::open(&filename);
    // each value represents whether the function succeeded or failed
    /*
    // this is too long
    let greeting_file = match greeting_file_result {
        // if the result is `Ok`, the function succeeded, and we can use the value inside
        Ok(file) => file,
        // if the result is `Error`, the error value is available
        // Err(error) => panic!("Problem opening the file: {error:?}"),

        // with `error.kind()`, we can decide action based on error
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Problem creating the file: {error:?}"),
            },
            _ => panic!("Problem opening the file: {error:?}"),
        },
    };
    */

    // there is a `unwrap_or_else` for this
    let greeting_file = greeting_file_result.unwrap_or_else(|error| {  // this is `closure`
        match error.kind() {
            ErrorKind::NotFound => {
                File::create(&filename).unwrap_or_else(|e| {
                    panic!("Problem creating the file: {e:?}");
                })
            },
            _ => panic!("Problem opening the file: {error:?}"),
        }
    });

    // we can also use `unwrap()` which calls the `panic!` by itself
    let greeting_file = File::open("hello.txt").unwrap();
    // or we can use `expect()` to give the message of `panic!`
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

}

// functions returning `Result`
// feels bit like `Option`, or java `Optional`
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// the `?` makes the function return
// the `Err` that would have been the value of an expression
// otherwise, the `Ok` is unpacked and becomes the expression's value
fn read_username_from_file_hookop() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_from_file_hook_chain() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_to_string() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn hook_option(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
