// external dependency
use rand::prelude::*;

fn main() {
    let secret_number = rand::rng().random_range(1..=100);
    println!("{secret_number}")
}
