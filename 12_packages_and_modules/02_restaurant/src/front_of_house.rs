pub mod hosting {
    pub fn add_to_waitlist() {}

    pub fn seat_at_table() {}

    fn private_fn() {}
}

mod private_module {
    // even when the inner function is public
    pub fn pub_in_private() {}
}

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}