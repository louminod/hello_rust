fn use_slice(slice: &mut [i32]) {
    println!("first element = {}, length = {}", slice[0], slice.len());
    slice[0] = 4321;
}

pub(crate) fn run() {
    let mut data = [1, 2, 3, 4, 5];
    use_slice(&mut data[1..4]);
    use_slice(&mut data);
    println!("{:?}", data);
}
