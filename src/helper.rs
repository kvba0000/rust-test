use std::{env::args_os, io::stdin};

pub fn get_option_val(name: &str) -> Option<String> {
    let num_index = get_option(name);

    if let Some(index) = num_index {
        let val = args_os().nth(index + 1);
        if let Some(v) = val {
            return v.into_string().ok()
        }
    }
    
    None
}

pub fn get_option(name: &str) -> Option<usize> {
    let arg_name = format!("--{}", name);
    args_os().position(|arg| *arg.to_ascii_lowercase() == *arg_name)
}


pub fn get_num_from_input(prompt: &str) -> f64 {
    loop {
        let mut choice = String::new();

        get_string_from_input(&prompt, &mut choice);

        match choice.trim().parse() {
            Ok(n) => break n,
            Err(_) => println!("Couldn't get the number! Try again!")
        };
    }
}

pub fn get_bool_from_input(prompt: &str) -> bool {
    let prompt = format!("{} (y/N)", prompt);
    let mut choice = String::new();

    get_string_from_input(&prompt, &mut choice);

    choice.trim().to_lowercase() == "y"
}

pub fn get_string_from_input(prompt: &str, buf: &mut String) {
    println!("{}", &prompt);

    stdin().read_line(buf).unwrap_or_default();
    *buf = buf.trim().to_string();
}
