use std::num::Wrapping;

pub mod array;
pub mod core;
pub mod elements;
pub mod events;
pub mod mvalue;
pub mod rgba;
pub mod string;
pub mod string_view;
pub mod vector;

#[macro_use]
pub mod log;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod natives;

pub fn hash(text: &str) -> u32 {
    let bytes = text.as_bytes();
    let mut num: Wrapping<u32> = Wrapping(0u32);

    for n in bytes {
        num += Wrapping(*n as u32);
        num += num << 10;
        num ^= num >> 6;
    }

    num += num << 3;
    num ^= num >> 11;

    (num + (num << 15)).0
}
