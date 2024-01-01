#[no_mangle]
pub extern "C" fn b() {
    let mut b = 1;
    print!("a: {}\n", b);
    b = 2;
    print!("a: {}\n", b);
}