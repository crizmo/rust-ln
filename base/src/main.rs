extern crate dlopen;
#[macro_use]
extern crate dlopen_derive;
use dlopen::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
struct Api {
    a: fn(arg: i32) -> u32,
    b: fn(arg: i32) -> u32,
}

fn main() {
    let cont: Container<Api> =
        unsafe { Container::load("libmain.so") }.expect("Could not open library or load symbols");

    println!("Enter the function name (a, b, c, etc.):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    let function_name = input.trim();

    match function_name {
        "a" => {
            let _ = cont.a(42);
        }
        "b" => {
            let _ = cont.b(42);
        }
        _ => {
            println!("Unknown function name");
        }
    }
}

