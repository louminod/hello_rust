#![allow(dead_code)]
#![allow(unused_imports)]

pub(crate) fn run() {
    let a = 123;

    {
        let b = 456;
        println!("b = {}", b);

        let a = 777;
        println!("a = {}", a);
    }

    println!("a = {}", a);
    // println!("b = {}", b);
}
