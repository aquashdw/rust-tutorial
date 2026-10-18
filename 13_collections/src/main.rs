use std::collections::HashMap;

fn main() {
    vectors();
    strings();
    hashmaps();
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

fn strings() {
    // creating an empty `String`
    let mut s = String::new();

    // string literal to `String`
    let data = "initial contents";
    let s = data.to_string();
    // in one-line
    let s = "initial contents".to_string();
    // with `::from`
    let s = String::from("initial contents");
    let s = String::from(data);

    // hello in various languages; strings are utf-8 encoded
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // mutable string can be updated
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    // '&str' can be appended
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    // `s2`'s ownership is not taken
    println!("s1: {s1}, s2: {s2}");

    // characters can be appended with `push`
    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    // string can use `+`
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;
    println!("{s3}");
    // but this makes `s1` invalid because of the `add` method
    // println!("{s1}");  // Value used after being moved

    // for multiple strings, use `format!`
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let s1 = String::from("hi");
    // strings cannot be accessed with indexing
    // let h = s1[0];  // the type `str` cannot be indexed by `{integer}`

    // strings in rust are actually `Vec<u8>` wrapper, one byte per index
    // rust uses utf-8 to encode strings, making the following string `hello`'s length 4
    let hello = String::from("Hola");
    // whereas the following string is made of Cyrillic, making them require 2 bytes per letter
    // making `hello`'s length 24 (twice the actual character count)
    let hello = String::from("Здравствуйте");
    // and trying to access a character with indexing will result in unpredictable results


    // we can slice strings
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("sliced 0..4: {s}");

    // but must be cautious because some characters are 1byte and some are 2bytes
    // and below results in a runtime error
    /*
    let s = &hello[0..1];  // end byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2 of string)
    println!("sliced 0..1: {s}");
    */

    let hello = "hello";
    let s = &hello[0..1];
    println!("sliced 0..1: {s}");

    let hello = "hello안녕";
    let len = hello.bytes().len();
    println!("{len}");
    let s = &hello[4..8];
    println!("sliced 4..6: {s}");

    // we can iterate with `.chars()`
    for c in "Здравствуйте".chars() {
        print!("{c}");
    }
    println!();

    // or `.bytes()`
    for c in "Здравствуйте".bytes() {
        print!("{c} ")
    }
    // when using `.bytes()`, valid Unicode may be made up of more than 1 byte.
    println!();
}

fn hashmaps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{team_name} scored: {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let mut map = HashMap::new();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    // owned values like `String`'s ownership is moved when used for `HashMap`
    map.insert(field_name, field_value);
    // println!("{field_name}");   // Value used after being moved
}
