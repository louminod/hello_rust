#![allow(dead_code)]
#![allow(unused_imports)]

use std::mem;

pub(crate) fn run() {
    // u = unsigned, 8 bits, 0 - 255
    let a: u8 = 123;
    println!("a = {}", a);

    // i = signed, 8 bits, -128 - 127
    let mut b: i8 = 0;
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    // Auto signed
    let mut c = 123456789;
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));

    // usize isize
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    let d: char = 'x';
    println!("{} is a char, takes up {} bytes", d, mem::size_of_val(&d));

    // f32 f64 IEEE754 signed!
    let e: f32 = 2.5;
    println!("{}, takes up {} bytes", e, mem::size_of_val(&e));

    let g: bool = false;
    println!("{}, takes up {} bytes", g, mem::size_of_val(&g));
}
