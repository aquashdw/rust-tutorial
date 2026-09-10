const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    immutable();
    mutable();
    println!("{THREE_HOURS_IN_SECONDS}");
    shadowing();
}

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

fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // OK
    let spaces = "   ";
    let spaces = spaces.len();

    // BAD
    let mut spaces = "   ";
    // spaces = spaces.len();  // Type mismatch
}
