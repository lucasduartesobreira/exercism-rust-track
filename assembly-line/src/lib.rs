// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const LINEAR_CONSTANT: f64 = 221.0;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let items_per_hour = 221.0 * f64::from(speed);
    let success_rate: f64 = get_success_rate(speed);

    items_per_hour * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let items_per_hour = LINEAR_CONSTANT * f64::from(speed);
    let success_rate: f64 = get_success_rate(speed);

    let working_items_per_hour = items_per_hour * success_rate;
    let working_items_per_minute = working_items_per_hour / 60.0;

    working_items_per_minute.floor() as u32
}

fn get_success_rate(speed: u8) -> f64 {
    match speed {
        x if x <= 4 => 1.0,
        x if x <= 8 => 0.90,
        x if x <= 10 => 0.77,
        _ => 0.0,
    }
}
