enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    }
}

 fn main() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Right;
    move_avatar(avatar1);
    move_avatar(avatar2);

    move_avatar(Movement::Up);
    move_avatar(Movement::Down);

    // Pattern Matching
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
