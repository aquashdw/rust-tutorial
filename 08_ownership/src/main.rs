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
}
