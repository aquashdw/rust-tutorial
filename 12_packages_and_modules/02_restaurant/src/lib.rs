mod front_of_house {
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
}

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
}
