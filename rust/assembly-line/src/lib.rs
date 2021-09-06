// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        1..=4 => {
            let rate: f64 = 221.0;
            speed as f64 * rate 
        },
        5..=8 => {
            let rate: f64 = 198.9;
            speed as f64 * rate 
        },
        9..=10 => {
            let rate: f64 = 170.17;
            speed as f64 * rate
        },
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    match speed {
        1..=4 => {
            let rate: u32 = 221;
            speed as u32 * rate / 60 as u32
        },
        5..=8 => {
            let rate: u32 = 198;
            speed as u32 * rate / 60 as u32
        },
        9..=10 => {
            let rate: u32 = 170;
            speed as u32 * rate / 60 as u32
        },
        _ => 0,
    }
}
