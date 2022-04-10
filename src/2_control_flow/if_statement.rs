#![allow(dead_code)]

pub(crate) fn run() {
    let temp = 25;

    if temp > 30 {
        println!("hot");
    } else if temp < 10 {
        println!("cold");
    } else {
        println!("normal");
    }

    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("today is {}", day);
}