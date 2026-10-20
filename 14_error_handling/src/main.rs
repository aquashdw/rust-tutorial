use std::io;

fn main() {
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
