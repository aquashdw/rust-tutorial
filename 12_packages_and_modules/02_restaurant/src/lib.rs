// the contents of this module is moved to `front_of_house.rs`
/*mod front_of_house {
    // with the `pub` keyword the makes the module public
    pub mod hosting {
        // `pub` keyword makes functions public as well
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}

        // without pub this function cannot be called
        // even if the module is public
        fn private_fn() {}
    }

    // without `pub` nothing in the module is not accessible
    mod private_module {
        // even when the inner function is public
        pub fn pub_in_private() {}
    }

     mod serving {
         fn take_order() {}

         fn serve_order() {}

         fn take_payment() {}
     }
}*/
mod front_of_house;
// this `use` brings `front_of_house::hosting` to this file's `front_of_house`
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // public functions can be called absolute or relative

    // crate is the `lib.rs` or `main.rs` in `src/`
    // which acts as the root for abs path
    crate::front_of_house::hosting::add_to_waitlist();
    // module `front_of_house` is the same level as this function,
    // so it's a sibling which don't have to be public
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // using super, we can call relative parent path
        super::deliver_order();
    }

    fn cook_order () {}

    // we can use `pub` in front of structs as well
    pub struct Breakfast {
        // the struct's fields must be designated public separately
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // methods must be designated public as well
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // when enums are public, all it's variants are public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_brunch_cafe() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // we can use struct's public fields
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // we can't use struct's private fields
    // // error: Field `seasonal_fruit` in struct `back_of_house::Breakfast` is private
    // println!("seasonal fruit served: {}", meal.seasonal_fruit);
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// using `use` keyword can create a shortcut for modules
// use crate::front_of_house::hosting;

pub fn eat_at_diner() {
    hosting::add_to_waitlist();
}

mod customer {
    // the `use` is limited to the scope it was occurred
    // so the inner module have to add `use` separately
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}


// while for functions, it's idiomatic to specify the parent module when using `use`
// to reveal the function is being brought from somewhere else.
// below works, but not recommended
// use crate::front_of_house::hosting::add_to_waitlist;


// whereas structs, enums, and other items, people use `use` to specify the full path,
// unless we use two different things with the same name,
// such as `std::fm::Result` and `std::io::Result`
// it's just a custom people have grown to.
use std::collections::HashMap;

fn use_for_other_objects() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}


// or we can use the `as` keyword to introduce alias.
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;

/*// we can add `pub` in front of `use` to re-export
pub use crate::front_of_house::hosting;
*/