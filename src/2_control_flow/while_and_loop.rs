#![allow(dead_code)]

pub(crate) fn run() {
    let mut x = 1;

    // while loop
    while x < 1000 {
        x *= 2;

        if x == 64 { continue; }

        println!("x = {}", x);
    }

    // loop
    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {}", y);

        if y == 1 << 10 { break; }
    }

    // For loop
    // For x from 1 to 10 inclusive
    for x in 1..11 {
        if x == 3 { continue; }
        if x == 8 { break; }
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}