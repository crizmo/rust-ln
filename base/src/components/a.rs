#[no_mangle]
pub extern "C" fn a(x: i32) -> u32 {
    println!("Hello from Rust in a.rs and x is {}", x);
    0 // Return a default value of 0
}