// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    const CARS_PRODUCED_EACH_HOUR_AT_LOWEST_SPEED: u8 = 221;

    let result = f64::from(speed) * f64::from(CARS_PRODUCED_EACH_HOUR_AT_LOWEST_SPEED);

    match speed {
        1..=4 => result,
        5..=8 => result * 0.9,
        9..=10 => result * 0.77,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / f64::from(60)) as u32
}
