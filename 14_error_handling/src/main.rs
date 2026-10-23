use std::fs::File;
use std::io;
use std::io::ErrorKind;

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
    let greeting_file_result = File::open("hello.txt");
    // each value represents whether the function succeeded or failed
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
}
