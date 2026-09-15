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

// some_string comes into scope
fn takes_ownership(some_string: String) {
    println!("{some_string}!");
}  // some_string is freed

// some_integer comes into scope
fn makes_copy(some_integer: i32) {
    println!("{some_integer}!!!");
}  // some_integer goes out.....which is it.
