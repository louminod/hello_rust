#![allow(dead_code)]

pub(crate) fn run() {
    let country_code = 5;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid"
    };

    print!("The country with the code {} is {}", country_code, country);

    let x = false;

    let s = match x {
        true => "yes",
        false => "no"
    };

    println!("{}", s);
}