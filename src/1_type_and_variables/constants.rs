#![allow(dead_code)]

// NO FIXED ADDRESS
pub const MEANING_OF_LIFE: u8 = 42;

static mut Z: i32 = 123;

pub(crate) fn run() {
    unsafe {
        Z = 777;
        println!("{}", Z);
    }
}