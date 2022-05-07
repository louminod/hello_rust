pub(crate) fn run() {
    let mut vec = vec![3, 2, 1];

    for x in &vec {
        println!("{}", *x);
    }

    for x in vec.iter() {
        println!("we got {}", x);
    }

    for x in vec.iter_mut() {
        *x += 2;
    }
    println!("{:?}", vec);

    for x in vec.iter().rev() {
        println!("in reverse {}", x);
    }

    let mut vec2 = vec![1, 2, 3];
    vec2.extend(vec);
    println!("{:?}", vec2);
}