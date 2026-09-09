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
}
