// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        1..=4 => {
              (speed * 221).into() 
        }
        5..=8 => { 
              (speed * 221).into() 
        }
        9..=10 => { 
              (speed * 221).into() 
        }
        _ => 0.0
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    unimplemented!("calculate the amount of working items at speed: {}", speed)
}
