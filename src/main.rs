mod helper;
mod examples {
    pub mod celsius;
    pub mod fibonacci;
    pub mod ownership;
    pub mod user;
    pub mod rectangle;
    pub mod ip;
    pub mod request_test;
}


use std::process::exit;

use crate::helper::get_option_val;
use crate::examples::{
    fibonacci::fibonacci_init,
    celsius::celsius_init,
    ownership::ownership_init,
    user::user_init,
    rectangle::rectangle_init,
    ip::ip_init,
    request_test::request_init
    
};

const OPTIONS: &[(&str, &str, fn())] = &[
    ("fibonacci", "Calculate the Fibonacci sequence", fibonacci_init),
    ("celsius", "Convert Celsius to Fahrenheit", celsius_init),
    ("ownership", "Ownership borrowing and moving", ownership_init),
    ("user", "User register logic showed on structs", user_init),
    ("rectangle", "Rectangle area calculation with structs", rectangle_init),
    ("ip", "IP Detection and object building with enums and structs", ip_init),
    ("request", "Making simple request with reqwest crate", request_init)
];

fn show_help() {
    println!("Invalid option! You can use the following options:\n");
    for ( name, description, _ ) in OPTIONS {
        println!("\t--example {} ({})", name, description);
    }
    exit(1);
}

fn main() {

    let example = get_option_val("example").unwrap_or_default();

    let func = OPTIONS.iter()
        .find(|opt| *opt.0 == *example.as_str())
        .map_or(show_help as fn(), |opt| opt.2);

    func()
}
