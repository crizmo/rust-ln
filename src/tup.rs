pub fn main() {
    let person: (&str, &str, i32) = ("Kurizu", "Tokyo", 27);
    println!("{} is {} years old", person.0, person.2);
    println!("{0} is from {1}", person.0, person.1);
}
