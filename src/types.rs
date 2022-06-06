/*
Primitive types:
    bool
    char
    i8 , i16 , i32 , i64
    u8 , u16 , u32 , u64
    f32 , f64
    string
    tuple
    array
    slice
    pointer
    reference
    raw pointer
    fnction pointer
    enum
    struct
    trait
    impl
    trait bound
    type alias
    macro
    macro

*/

// Rust is a statically typed language.

 fn main() {
    let x = 1;
    // Default x as a variable of type i32.

    let y = 2.5;
    // Default y as a variable of type f64.

    let z: i64 = -1;
    // Explicitly define z as a variable of type i64.

    let a: i64 = 4545445454545;
    println!("Max i64: {}", std::i64::MAX);
    println!("Min i64: {}", std::i64::MIN);

    let b: bool = true;

    // Get boolean from a expression.
    let c: bool = 5 < 4;

    let character: char = 'a';
    let face: char = '\u{1F600}';

    println!("{:?}", (x, y, z, a, b, c, character, face));
}
