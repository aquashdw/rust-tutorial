fn main() {
    {
        // immutable string literal (in stack)
        let s = "hello";
        println!("{s}");  // variable valid within scope
    }
    // println!("{s}");  // variable is not valid after scope end

    {
        // mutable `String` (allocated in heap)
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{s}");
    }  // data is freed when scope ends

    {
        // bind `5` to `x`,
        let x = 5;
        // then `x` to `y`
        let y = x;
        // types with fixed size are created directly in stack
        // and `x` and `y` are different variables using different places in memory
        println!("{x}, {y}");

        // similar to above, except on the stack only reference is saved;
        // actual string data are saved on the heap
        let s1 = String::from("hello");
        // and `=` copies reference not heap data
        let s2 = s1;
        // meaning when this scope ends, `s1` and `s2` is freed.
        // since they are the same, it results in double free error

        // but rust uses ownership, making `s1` move to `s2`
        println!("{s2}");
        // which means following is invalid,
        // println!("{s1}");  // Value used after being moved [E0382]
    }  // but no double free, safely closing the scope

    {
        let mut s = String::from("hello");
        println!("{s}, world!");
        // data is freed on reassign as well
        s = String::from("ahoy");
        println!("{s}, world!");
    }

    {
        // s comes into scope
        let s = String::from("hello");
        // when passed as argument, it moves to the function
        takes_ownership(s);
        // meaning it's not valid outside (here)
        // println!("{s}");  // Value used after being moved [E0382]

        // x comes into scope
        let x = 5;
        // i32 implements Copy trait, value is copied to function
        makes_copy(x);
        // meaning it's still valid outside (here)
        println!("{x}~~~");
    }
    {
        // return value of gives_ownership is moved to s1
        let s1 = gives_ownership();
        println!("{s1}");

        // s2 comes into scope
        let s2 = String::from("hello");

        // s2 moves into function,
        // then the return value is moved to s3
        let s3 = takes_and_gives_back(s2);
        println!("{s3}");
    }  // s1, s3 is dropped.

    {

        let s1 = String::from("hello");
        // using tuples can return multiple values,
        // but the argument is moved to function
        let (s2, len) = calculate_length_tuple(s1);
        println!("length of '{s2}': {len}");

        let s1 = String::from("hello");
        // to avoid moving arguments to function,
        // we can pass reference
        let len = calculate_length(&s1);
        println!("length of '{s1} is {len}'");

        // change function does nothing, check below
        change_fail(&s1);

        // to let a function modify borrowed value, the variable must be mutable
        let mut s1 = String::from("hello");
        change_with_mut(&mut s1);
        println!("{s1}");

        let r1 = &mut s1;
        // let r2 = &mut s1;  // cannot borrow `s1` as mutable more than once at a time
        println!("'{r1}' only");

        let mut s2 = String::from("Hello");
        // unless its in a new scope
        {
            let r2 = &mut s2;
            r2.push_str(", World!");
            println!("{r2}");
        }  // since its freed after close
        let r1 = &mut s2;
        println!("'{r1}'");

        // but should not create mutable reference if there is already an immutable one
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        // let r3 = &mut s;  // cannot borrow `s` as mutable because it is also borrowed as immutable
        println!("{r1}, {r2}");

        // immutable reference's scope lasts til when 'it was last used' (the above `println!` in this case)
        // so here it's okay
        let r3 = &mut s;
        r3.push_str(", world!");
        println!("{r3}");
    }


}

// some_string comes into scope
fn takes_ownership(some_string: String) {
    println!("{some_string}!");
}  // some_string is freed

// some_integer comes into scope
fn makes_copy(some_integer: i32) {
    println!("{some_integer}!!!");
}  // some_integer goes out.....which is it.


fn gives_ownership() -> String {
    // some_string comes into scope
    let some_string = String::from("yours");
    // some_string is returned and moves out to calling expression
    some_string
}

// a_string comes into scope
fn takes_and_gives_back(a_string: String) -> String {
    // a_string is returned and moves out to calling expression
    a_string
}

// returns tuple
fn calculate_length_tuple(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// or use borrowed reference
fn calculate_length(s: &String) -> usize {
    s.len()
}  // reference is just reference, no ownership

// but reference doesn't allow modification
fn change_fail(some_str: &String) {
    // some_str.push_str(", world!");  // Cannot borrow immutable local variable `some_str` as mutable
    println!("'{some_str}' is immutable");
}

// it should be notated its a reference of a mutable
fn change_with_mut(some_string: &mut String) {
    some_string.push_str(". world!");
}

