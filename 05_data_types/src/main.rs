use std::io;

fn main() {
    numbers();
    booleans();
    characters();
    tuples();
    arrays();
}


fn numbers() {
    // signed & unsigned int, up to 8 ~ 128 bits
    let signed16: i16 = 42;
    let unsigned32: u32 = 42;

    // architecture dependent int
    let signed_dependent: isize = 42;
    let unsigned_dependent: usize = 42;

    // floating point
    // default: double precision
    let double = 2.0;
    let float: f32 = 3.0;

    // numeric operations
    let sum = 5 + 10;
    println!("5 + 10 = {sum}");
    let difference = 95.9 - 4.3;
    println!("95.9 - 4.3 = {difference}");
    let product = 4 * 30;
    println!("4 * 30 = {product}");
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {quotient}");
    let truncated = -5 / 3;
    println!("-5 / 3 = {truncated}");
    let remainder = 43 % 5;
    println!("43 % 5 = {remainder}");

}

fn booleans() {
    let t = true;
    let f: bool = false;
    let and = t && f;
    let or = t || f;
    println!("{and}");
    println!("{or}");
}

fn characters() {
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("{c} {z} {heart_eyed_cat}");
}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("{x}, {y}, {z}");

    let (x, y, z) = tup;
    println!("{x}, {y}, {z}");
}

fn arrays() {
    let arr = [1, 2, 3, 4, 5];
    let a1 = arr[1];
    println!("{a1}");

    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    let monday = days[0];
    println!("{monday}");

    let arr = [3; 5];
    let arr_len = arr.len();
    let three = arr[2];
    println!("{three} {arr_len}");
    // index_out_of_bounds();
}

fn index_out_of_bounds() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index.trim()
        .parse()
        .expect("Input was not a uint");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
