struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

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
}
