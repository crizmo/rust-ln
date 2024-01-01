#[no_mangle]
pub extern "C" fn a(x: i32) -> u32 {
    println!("Hello from Rust in a.rs");
    x as u32
}