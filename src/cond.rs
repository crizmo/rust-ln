pub fn main() {
    let age = 19;
    let check_id = true;
    let knows_person_age = true;
    let person = ("Kurizu", "Tokyo", 27);

    if age >= 18 && check_id || knows_person_age || person.0 == "Kurizu" {
        println!("You can drink beer!");
    } else if age >= 18 && !check_id {
        println!("You can drink beer but first you need a id!");
    } else {
        println!("You can't drink beer!");
    }

    // Shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("{}", is_of_age);
}
