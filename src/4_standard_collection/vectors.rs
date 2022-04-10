pub(crate) fn run() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("vec = {:?}", vec);

    vec.push(4);

    println!("vec = {:?}", vec);

    let idx: usize = 0;

    vec[idx] = 321;

    println!("vec[0] = {}", vec[idx]);

    // Option
    match vec.get(6) {
        Some(x) => println!("vec[6] = {}", x),
        None => println!("error, no such element"),
    }

    let last_elem = vec.pop();
    println!("last_elem = {:?}", last_elem);

    for x in &vec {
        println!("{}", x);
    }

    while let Some(x) = vec.pop() {
        println!("{}", x);
    }
}
