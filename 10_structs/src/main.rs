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
    // the struct update syntax moves user1's ownership, meaning we can't use user1 afterwards
    // which also goes for the above, where we manually use user1's fields for user2
    // let user1_email = user1.email;  // Value used after being moved

    // we instantiate tuple structs like this
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
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
