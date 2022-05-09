pub fn run() {
    greeting("Hi", "Kurizu");
    let get_sum = add(1, 2);
    println!("Sum: {}", get_sum);

    // Closure
    let z: i32 = 3;
    let get_sum = |x: i32, y: i32| x + y; // can do x + y + z because of closure
    let get_sum_closure = |x: i32, y: i32| x + y + z;
    println!("C Sum: {}", get_sum(1, 2));
    println!("C Sum with z: {}", get_sum_closure(1, 2));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}!", greet, name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
