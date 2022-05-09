pub fn run() {
    let age = 19;
    let check_id = true;
    let knows_person_age = true;
    let person = ("Kurizu", "Tokyo", 27);

    println!("For loop");
    for i in 0..3 {
        println!("{}", i);
    }

    println!("While loop");
    let mut i = 0;
    while i < 3 {
        println!("{}", i);
        i += 1;
    }

    println!("Loop");
    i = 0;
    loop {
        println!("{}", i);
        i += 1;
        if i == 3 {
            break;
        }
    }
}
