pub fn main() {
    let mut str = String::from("Hello World"); // mutable string
    println!("{}", str);
    str = String::from("Kurizu");
    println!("{}", str);

    // String concatenation
    let str1 = String::from("Hello");
    let str2 = String::from("World");
    let str3 = str1 + " " + &str2;
    println!("{}", str3);

    // Get the length of a string
    println!("Length: {}", str3.len());

    // Check if a string is empty
    println!("Empty: {}", str3.is_empty());

    // Get a character
    println!("First character: {}", str3.chars().next().unwrap());

    // Get a string slice
    println!("First word: {}", str3.split_whitespace().next().unwrap());

    // Push
    str.push('!');
    println!("{}", str);

    // Pop
    str.pop();
    println!("{}", str);

    // Get a string slice
    let str_slice = &str[0..5];
    println!("{}", str_slice);

    // Capacity
    println!("Capacity: {}", str.capacity());

    // Truncate
    str.truncate(5);
    println!("{}", str);

    // To lowercase
    println!("To lowercase: {}", str.to_lowercase());

    // To uppercase
    println!("To uppercase: {}", str.to_uppercase());

    // Contains
    println!("Contains 'uriz': {}", str.contains("uriz"));
    println!("The string is: {}", str);

    // Replace
    println!("After Replace: {}", str.replace("Kuriz", "Kurizu"));

    // Split
    let str_split = "Hello World";
    let mut str_split_iter = str_split.split_whitespace();
    println!("{}", str_split_iter.next().unwrap());

    // Loop through a string
    for str_split in str_split_iter {
        println!("{}", str_split);
    }

    // Loop through a string slice
    let str_slice = &str[0..5];
    for str_slice in str_slice.chars() {
        println!("{}", str_slice);
    }

    // Create a string with capacity
    let mut str_with_capacity = String::with_capacity(10);
    str_with_capacity.push('a');
    str_with_capacity.push('b');
    println!("{}", str_with_capacity);

    // Assertion testing
    let str_assert = String::from("Hello World");
    assert_eq!(str_assert, "Hello World");
}
