pub fn run() {
    println!("Hello world!");

    // Formatting
    println!("Number {}", 1);
    println!("{} is a {}", "Kurizu", "programmer");

    // Positional Arguments
    println!("{0} likes to {1}", "Kurizu", "code");
    println!("{1} likes to {0}", "Kurizu", "code");

    // Named Arguments
    println!(
        "{name} likes to {activity}",
        name = "Kurizu",
        activity = "code"
    );

    // Placeholders traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholders for debug traits
    println!("{:?}", (12, true, "Kurizu"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
