 fn main() {
    let name = "Kurizu";
    let mut age = 27;
    println!("My name is {0}, {0} is {1} years old", name, age);

    age = 28;
    println!("My name is {0}, {0} is {1} years old", name, age);

    // const
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, my_age) = ("Kurizu", 27);
    println!("{} is {} years old", my_name, my_age);
}
