struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


// these are tuple structs
// useful to give tuple names, when field naming is to verbose
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


// structs that don't have any fields are called unit-like structs (like unit types)
struct AlwaysEqual;


fn main() {
    // immutable struct
    let user1 = User {
        active: true,
        username: String::from("alex"),
        email: String::from("alex@naver.com"),
        sign_in_count: 1,
    };
    // can retrieve fields with dot
    let user1_email = user1.email;
    println!("{user1_email}");
    // cannot alter immutable struct's field
    // user1.email = String::from("alex@gmail.com");  // Cannot assign a new value to `field of immutable binding

    // if the struct is mutable, the fields are as well
    let mut user1 = User {
        active: true,
        username: String::from("alex"),
        email: String::from("alex@naver.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("alex@gmail.com");
    let user1_email = user1.email;
    println!("{user1_email}");

    let user2 = build_user(String::from("brad@gmail.com"), String::from("brad"));
    let user2_email = user2.email;
    println!("{user2_email}");

    // say we need to make user2 from user1
    /*
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("alex@naver.com"),
        sign_in_count: user1.sign_in_count,
    };
    */
    // we can use struct update syntax to unpack fields from another struct instance
    let user2 = User {
        email: String::from("alex@naver.com"),
        ..user1  // no trailing comma allowed
    };
    // the struct update syntax moves user1's ownership, meaning we can't use user1 afterward
    // which also goes for the above, where we manually use user1's fields for user2
    // let user1_email = user1.email;  // Value used after being moved

    // we instantiate tuple structs like this
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // unit-like structs instantiate like this
    let subject = AlwaysEqual;

    rectangles();
}

fn build_user(email: String, username: String) -> User {
    /*
    User {
        active: true,
        // this is a bit tedious...
        username: username,
        email: email,
        sign_in_count: 1
    }
     */
    User {
        active: true,
        // so we can use the field init shorthand
        // a bit like JavaScript huh
        username,
        email,
        sign_in_count: 1
    }
}

// instead of a new project, I'll create a separate function here
// for references
// [an example program using structs](https://doc.rust-lang.org/book/ch05-02-example-structs.html)
fn rectangles() {
    // without tuples or structs, we have to represent rectangles like this
    let width1 = 30;
    let height1 = 50;

    // and the signature of an area function would be like `area(u32, u32)`
    println!("The area of the rectangle is {} square pixels.", area1(width1, height1));
    // ...but the parameters of the area1 function isn't clear about how they are inter-related
    // (the width and height of *a* rectangle).

    // if we use tuples, we can group the values into one variable
    let rect1 = (30, 50);  // width, height
    println!("The area of the rectangle is {} square pixels.", area2(rect1));
    // which makes sense, but the tuple still don't have any knowledge of
    // what's the width and what's the height. one must memorize idx 0 is width, idx 1 is height.

    // if we use structs, we can label the data.
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", area(&rect1));

}

// if we use structs, we can label the data.
struct Rectangle {
    width: u32,
    height: u32,
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
