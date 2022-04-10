// 32 bits
union IntOrFloat {
    i: i32,
    f: f32,
}

fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life value");
            }
            IntOrFloat { f } => {
                println!("value = {}", f);
            }
        }
    }
}

pub(crate) fn run() {
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 123;

    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    process_value(IntOrFloat { i: 5 });
}