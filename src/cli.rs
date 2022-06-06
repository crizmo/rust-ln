use std::env;
use std::io::stdin;

 fn main() {
    let arg: Vec<String> = env::args().collect();
    let command = arg[1].clone();

    // let name = "Kurizu";

    let mut name = String::new();
    stdin()
        .read_line(&mut name)
        .ok()
        .expect("Failed to read line");

    // println!("Command: {}", command);
    if command == "hi" {
        println!("Hi {}How are you?", name);
        let mut state = String::new();
        stdin()
            .read_line(&mut state)
            .ok()
            .expect("Failed to read line");
        println!("You are {}", state);
    } else if command == "bye" {
        println!("Bye {}", name);
    } else {
        println!("Unknown command {}", command);
    }
}
