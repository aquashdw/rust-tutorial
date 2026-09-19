fn main() {
    let mut s = String::from("hello world");
    // word will get the value 5
    let word = first_word_noslice(&s);
    // this clears the String (equal to "")
    s.clear();
    // while `word` is still 5
    println!("{word}");
    // which is prone to danger because `s` is empty now

    // with slicing
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    // range syntax examples
    let slice = &s[0..2];
    // omit start idx
    let slice = &s[..2];

    let slice = &s[3..s.len()];
    // omit end idx
    let slice = &s[3..];


    // this works
    let word = first_word(&s);
    println!("the first word is: {word}");

    // this doesn't
    // s.clear();  // Cannot borrow immutable local variable `s` as mutable
}


// without slicing
fn first_word_noslice(s: &String) -> usize {
    // 1. convert to bytes
    let bytes = s.as_bytes();

    // 2. iter through bytes
    for (i, &item) in bytes.iter().enumerate() {
        // 3. if it's space, return the index
        if item == b' ' {
            return i;
        }
    }

    // 4. if not found, return length
    s.len()
}

// function that returns sliced string
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
