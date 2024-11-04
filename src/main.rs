mod helper;
mod examples {
    pub mod celsius;
    pub mod fibonacci;
    pub mod ownership;
}


use std::process::exit;

use crate::helper::get_option_val;
use crate::examples::{
    fibonacci::fibonacci_init,
    celsius::celsius_init,
    ownership::ownership_init
};

const OPTIONS: &[(&str, &str)] = &[
    ("fibonacci", "Calculate the Fibonacci sequence"),
    ("celsius", "Convert Celsius to Fahrenheit"),
    ("ownership", "Ownership borrowing and moving"),
];

fn main() {

    let example = get_option_val("example").unwrap_or_default();

    match example.as_str() {
        "fibonacci" => fibonacci_init(),
        "celsius" => celsius_init(),
        "ownership" => ownership_init(),
        _ => {
            println!("Invalid option! You can use the following options:\n");
            for ( name, description ) in OPTIONS {
                println!("--example {} ({})", name, description);
            }
            exit(1);
        }
    }

}
