const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn immutable() {
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6;  // error
    // println!("The value of x is: {x}");
}

fn mutable() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn main() {
    immutable();
    mutable();
    println!("{THREE_HOURS_IN_SECONDS}");
}
