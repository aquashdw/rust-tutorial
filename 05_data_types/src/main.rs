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

fn main() {
    numbers();
    booleans();
    characters();
}
