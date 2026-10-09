// external dependency
use rand::prelude::*;

// nested paths
// for items in the same crate we can nest them in curly brakets
use std::{cmp::Ordering, io, io::Write};

// Glob
use std::collections::*;

fn main() {
    let secret_number = rand::rng().random_range(1..=100);
    println!("{secret_number}")
}
