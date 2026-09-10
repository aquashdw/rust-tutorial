fn main() {
    println!("Hello, world!");
    another_function();
    print_labeled_measurement(5, 'h');
    curly_bracket_expr();
    let x = five();
    println!("The result of `five()` is: {x}");
    let x_plus_one = plus_one(x);
    println!("The result of x + 1 is: {x_plus_one}");
}

fn another_function() {
    println!("Another function.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn curly_bracket_expr() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

fn five() -> i32 {  // return type must be specified
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1  // must omit semicolon to make an expression
}


