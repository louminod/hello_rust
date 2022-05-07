pub(crate) fn run() {
    let s = "hello";

    for c in s.chars() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char);
    }

    // String => heap allocated
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);

    // &str <> String
    let u: &str = &letters;

    // concatenation
    // String + str
    let z = letters + "abc";

    let mut abc = String::from("hello world");
    let mut def = "hello world".to_string();

    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));
}