use crate::helper::{get_option, get_option_val};

use std::{process::exit, thread::sleep, time::Duration};

const FIBONACCI_MAX: u8 = 100;

const ARGS: &[(&str, &str, &str)] = &[
    ("goal", "--goal <number>", "Set the maximum Fibonacci number to calculate (default: 10000), Max: 255"),
    ("skiprest", "--skiprest", "Skip calculating numbers before the goal"),
];

fn calculate_fibonacci(num: &u8) -> u128 {
    // Using Binet's formula: Fₙ = (φⁿ - (-φ)⁻ⁿ)/√5 where φ = (1 + √5)/2
    let phi: f64 = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let sqrt5 = 5.0_f64.sqrt();
    
    // Calculate using floating point then round to nearest integer
    let n = f64::from(*num);
    let result = ((phi.powf(n) - ((-phi).powf(-n))) / sqrt5).round() as u128;
    
    if result >= u128::MAX {
        println!("Fibonacci number {} would overflow u128", &num);
        exit(1);
    }

    result
}

pub fn fibonacci_init() {
    if get_option("help").is_some() {
        for ( name, usage, description ) in ARGS { println!("{name} ({usage}) - {description}"); }
        exit(0); 
    }

    let goal: u8 = match get_option_val("goal").unwrap_or_default().parse() {
        Ok(v) => { v }
        Err(_) => {
            println!("Invalid --goal argument. Using default ({})", &FIBONACCI_MAX);
            sleep(Duration::from_secs(2));

            FIBONACCI_MAX
        }
    };

    let skip_rest: bool = get_option("skiprest").is_some();
    
    for i in (if skip_rest { goal } else { 2 })..=goal {
        let num = calculate_fibonacci(&i);
        println!("Fibonacci number for {} is {}", i, num)
    }
}    