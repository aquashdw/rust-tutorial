fn main() {
    vectors();
}

fn vectors() {
    // creating a vector instance
    let v: Vec<i32> = Vec::new();
    println!("{v:?}");

    // creating and initiating vector
    let v = vec![1, 2, 3];
    println!("{v:?}");

    // adding elements to a mutable vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{v:?}")
}
