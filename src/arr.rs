use std::mem;

 fn main() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    numbers[1] = 20;
    println!("{:?}", numbers);

    // Reverse the array
    numbers.reverse();
    println!("{:?}", numbers);

    //  get single value
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice = &numbers[1..4];
    println!("Slice: {:?}", slice);
}
