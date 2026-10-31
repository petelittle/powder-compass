use std::io::{self, Write};

fn main() {
    println!("Powder Compass ❄️");
    println!("A tiny snow-day scorecard for skiers and snowboarders.");
    println!();

    print!("Where are you riding? ");
    io::stdout().flush().expect("Could not show the question");

    let mut place = String::new();
    io::stdin()
        .read_line(&mut place)
        .expect("Could not read your answer");

    let place = place.trim();

    println!("Checking the snow at {place}...");
}
