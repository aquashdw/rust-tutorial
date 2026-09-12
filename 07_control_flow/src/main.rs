fn main() {
    if_statement(3);
    if_statement(7);
    falsy();
    if_else_if(5);
    if_else_if(20);
    if_else_if(35);
    if_expression(true);
    if_expression(false);
    
}

fn if_statement(number: i32) {
    if number < 5 {
        println!("{number} is less than 5");
    } else {
        println!("{number} is bigger or equal to 5");
    }
}

fn falsy() {
    // rust doesn't have truthy falsy
    let falsy = 0;
    // if falsy { println!("this don't run") }  // error
    if falsy != 0 {
        println!("falsy is {falsy}")
    }
}

fn if_else_if(number: i32) {
    if number > 30 {
        println!("its hot")
    } else if number < 10 {
        println!("its cold")
    } else {
        println!("its nice")
    }
}

fn if_expression(raining: bool) {
    let raining = if raining { "raining" } else { "not raining" };

    // let number = if true { 5 }  else { "6" };  // if / else expressions must have same type

    println!("it is {raining}.")
}
