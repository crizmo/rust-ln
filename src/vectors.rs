use std::mem;

 fn main() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
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
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice = &numbers[1..4];
    println!("Slice: {:?}", slice);

    // Get slice from a reference
    let slice_ref = &numbers[1..4];
    println!("Slice from a reference: {:?}", slice_ref);

    // loop through vector
    for x in numbers.iter() {
        println!("{}", x);
    }

    // loop through vector (mutable)
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}
