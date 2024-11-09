use crate::helper::get_num_from_input;

pub fn celsius_init() {
    let choice: f64 = get_num_from_input("Type celsius number!");
    let result: f64 = &choice * 1.8 + 32_f64;

    println!(
        "{} celsius is {} farhenheit!",
        &choice, &result
    )
}
