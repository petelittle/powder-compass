use std::io::{self, Write};

fn ask_number(question: &str, min: i32, max: i32) -> i32 {
    loop {
        print!("{question}");
        io::stdout().flush().expect("Could not show the question");

        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Could not read your answer");

        match answer.trim().parse::<i32>() {
            Ok(number) if (min..=max).contains(&number) => return number,
            _ => println!("Please enter a whole number from {min} to {max}."),
        }
    }
}

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
    let snow_cm = ask_number("Fresh snow in the last 24 hours (cm, 0-100): ", 0, 100);
    let temperature_c = ask_number("Temperature (°C, -60 to 20): ", -60, 20);
    let wind_kmh = ask_number("Wind (km/h, 0-200): ", 0, 200);
    let visibility = ask_number("Visibility (1 = poor, 5 = crystal clear): ", 1, 5);
    println!();
    println!("--- Powder Compass for {place} ---");
    println!("Fresh snow: {snow_cm} cm");
    println!("Temperature: {temperature_c}°C");
    println!("Wind: {wind_kmh} km/h");
    println!("Visibility: {visibility}/5");
}
