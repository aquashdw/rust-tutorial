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
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}
