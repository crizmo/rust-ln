#[no_mangle]
pub extern "C" fn b(x: i32) -> u32 {
    println!("Hello from Rust in b.rs");
    x as u32
}