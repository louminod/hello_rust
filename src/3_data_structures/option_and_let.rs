pub(crate) fn run() {
    let x = 3.0;
    let y = 2.0;

    // Option -> Some(v) | None
    let result = if y != 0.0 {
        Some(x / y)
    } else {
        None
    };

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("cannot divide by zero")
    }

    if let Some(z) = result {
        println!("result = {}", z);
    }

}