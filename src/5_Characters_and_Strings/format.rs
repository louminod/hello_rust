pub(crate) fn run() {
    let name = "Louis-Marie";
    let greeting = format!("hi, I'm {}, nice to meet you", name);
    println!("{}", greeting);

    let hello = "hello";
    let rust = "rust";
    let hello_rust = format!("{}, {}!", hello, rust);
    println!("{}", hello_rust);

    let runner = "run";
    let forest = "forest";
    let rfr = format!("{0}, {1}, {0}!", runner, forest);
    println!("{}", rfr);

    let info = format!(
        "the name's {last}. {first} {last}.",
        first = "james",
        last = "bond"
    );
    println!("{}", info);

    let mixed = format!(
        "{1} {} {0} {} {data}",
        "alpha",
        "beta",
        data = "delta"
    );
    println!("{}", mixed);
}