use rand::Rng;

use crate::helper::{get_num_from_input, get_string_from_input};

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    id: u64,
    username: String,
    active: bool,
    age: u8
}

fn build_user(username: String, age: u8) -> User {
    User {
        // In normal case uses we wouldn't use something like this before
        // ...it might cause collision in database
        id: rand::thread_rng().gen_range(u64::MIN..u64::MAX),
        username,
        active: true,
        age
    }
}

pub fn user_init() {
    let mut username: String = String::new();
    get_string_from_input("What should be your username?", &mut username);

    let age: u8 = get_num_from_input("How old are you?") as u8;

    let user = build_user(username, age);

    println!("{:#?}", &user);
}
