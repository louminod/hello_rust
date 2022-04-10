#![allow(dead_code)]
#![allow(unused_imports)]

use std::mem;

fn double_value(v: i32) -> i32 {
    v * 2
}

pub(crate) fn run() {
    let mut x = 3;
    x = double_value(x);
    x = 42;
}