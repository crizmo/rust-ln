struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tupl Struct
struct ColorTuple(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn main() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;
    println!("{} {} {}", c.red, c.green, c.blue);

    // Tupl Struct
    let mut ct = ColorTuple(255, 0, 0);
    ct.0 = 200;
    println!("{} {} {}", ct.0, ct.1, ct.2);

    // Struct
    let mut p = Person::new("John", "Doe");
    println!("{}", p.full_name());
    p.set_last_name("Smith");
    println!("{}", p.full_name());

    // // Tuple Struct
    let p_tuple = p.to_tuple();
    println!("{} {}", p_tuple.0, p_tuple.1);
}
