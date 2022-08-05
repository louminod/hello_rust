struct Person {
    name: String,
}

impl Person {
    fn new<S>(name: S) -> Person where S: Into<String> {
        Person { name: name.into() }
    }
}

pub(crate) fn run() {
    let john = Person::new("John");

    let name: String = "Jane".to_string();
    let jane = Person::new(name);
}