extern crate dlopen;
use dlopen::symbor::{Library, Symbol};
use std::io::{self, Write};

fn main() {
    let lib = { Library::open("libmain.so") }.expect("Could not open library");

    print!("Please enter the name of the function to call: ");
    io::stdout().flush().unwrap(); // Make sure the prompt is immediately visible

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    let symbol: Result<Symbol<unsafe extern "C" fn(i32) -> u32>, _> = unsafe { lib.symbol(input) };
    let func = match symbol {
        Ok(symbol) => {
            let func = symbol;
            Ok(Some(func))
        }
        Err(_) => Err("No such function"),
    };

    match func {
        Ok(Some(func)) => {
            unsafe { func(5) };
        }
        Ok(None) => println!("Function pointer is null"),
        Err(_) => println!("No such function: {}", input),
    }
}