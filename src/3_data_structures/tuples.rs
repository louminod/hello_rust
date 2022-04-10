fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

pub(crate) fn run() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);

    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("combined = {:?}", combined);
    println!("last element = {}", (combined.1).1);

    let ((_c, _d), (_e, f)) = combined;
    println!("f = {}", f);

    let foo = (true, 42.0, -1i8);
    println!("foo = {:?}", foo);

    let meaning = (42,);
    println!("meaning = {:?}", meaning);
}
