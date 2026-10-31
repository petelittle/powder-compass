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

fn snow_points(snow_cm: i32) -> i32 {
    match snow_cm {
        0 => 0,
        1..=4 => 10,
        5..=9 => 20,
        10..=19 => 30,
        20..=29 => 36,
        _ => 40,
    }
}

fn temperature_points(temperature_c: i32) -> i32 {
    match temperature_c {
        -12..=-3 => 20,
        -18..=-13 | -2..=0 => 16,
        -25..=-19 | 1..=2 => 10,
        3..=5 => 5,
        6.. => 0,
        _ => 6,
    }
}

fn wind_points(wind_kmh: i32) -> i32 {
    match wind_kmh {
        0..=10 => 20,
        11..=20 => 17,
        21..=35 => 12,
        36..=50 => 6,
        _ => 0,
    }
}

fn visibility_points(visibility: i32) -> i32 {
    match visibility {
        1 => 2,
        2 => 6,
        3 => 11,
        4 => 16,
        _ => 20,
    }
}

fn verdict(score: i32, temperature_c: i32, wind_kmh: i32) -> &'static str {
    if temperature_c <= -30 || wind_kmh >= 70 {
        "Red-flag conditions — official advice first"
    } else {
        match score {
            85..=100 => "Champagne-powder alert",
            70..=84 => "A proper powder day",
            55..=69 => "Worth the early alarm",
            40..=54 => "A mixed mountain bag",
            25..=39 => "Cruisy, character-building day",
            _ => "Hot-chocolate conditions",
        }
    }
}

fn practical_tip(snow_cm: i32, temperature_c: i32, wind_kmh: i32, visibility: i32) -> &'static str {
    if temperature_c <= -30 {
        "Do not treat this as a normal snow day: check official warnings, cover every bit of skin, and seriously consider staying warm indoors."
    } else if wind_kmh >= 70 {
        "Extreme wind: expect major lift disruption and check the resort's status before travelling."
    } else if wind_kmh >= 36 {
        "Strong wind is the headline: check lift status before travelling, and pack a face covering."
    } else if temperature_c <= -18 {
        "It is seriously cold: take your warmest layers and plan a hot-drink stop."
    } else if temperature_c >= 3 {
        "Warm snow can turn heavy: relaxed pacing will help this afternoon."
    } else if visibility <= 2 {
        "Low visibility: low-light goggles and familiar runs are your friends."
    } else if snow_cm >= 20 {
        "There is plenty of fresh snow: arrive early before the best lines are tracked out."
    } else if snow_cm == 0 {
        "No fresh snow: a good day for carving practice, technique drills, and hunting sunny groomers."
    } else {
        "Balanced conditions: start with the runs you love and follow the best light."
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
    let score = snow_points(snow_cm)
        + temperature_points(temperature_c)
        + wind_points(wind_kmh)
        + visibility_points(visibility);

    println!();
    println!("--- Powder Compass for {place} ---");
    println!("Fresh snow: {snow_cm} cm");
    println!("Temperature: {temperature_c}°C");
    println!("Wind: {wind_kmh} km/h");
    println!("Visibility: {visibility}/5");
    println!();
    println!("Score: {score}/100");
    println!(
        "Verdict: {}",
        verdict(score, temperature_c, wind_kmh)
    );
    println!(
        "Tip: {}",
        practical_tip(snow_cm, temperature_c, wind_kmh, visibility)
    );
    println!();
    println!("Fun planning only — always check the resort's official conditions and avalanche information before you go.");
}
