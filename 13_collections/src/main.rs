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
    println!("{v:?}");

    let v = vec![1, 2, 3, 4, 5];

    // read vector elements with index
    let third = &v[2];
    println!("The third element is {third}");

    // read vector elements with `.get()`
    let third = v.get(2);
    // `.get()` returns an `Option` which can be used with `match`
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element")
    }

    let third = match v.get(10) {
        Some(val) => val,
        None => &0i32,
    };
    println!("Result is {third}");

    let third = v.get(2).unwrap_or_else(|| &0i32);
    println!("Result is {third}");

    let mut v = vec![1, 2, 3, 4, 5];
    // this is immutable borrow
    let first = &v[0];
    println!("The first element is {first}");
    // this is a mutable borrow
    v.push(6);
    // which doesn't let the below line to work correctly
    // println!("The first element is {first}");
    // unless it is borrowed again
    let first = &v[0];
    println!("The first element is {first}");
}
