use std::io;

pub fn evaluate_password() {
    println!("Evaluating the strength of a password...");

    let password = loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if !input.is_empty() {
            break input.to_string();
        }

        println!("Input cannot be empty.");
    };

    let mut has_lower = false;
    let mut has_upper = false;
    let mut has_number = false;
    let mut has_symbol = false;

    for c in password.chars() {
        if c.is_lowercase() {
            has_lower = true;
        } else if c.is_uppercase() {
            has_upper = true;
        } else if c.is_numeric() {
            has_number = true;
        } else {
            has_symbol = true;
        }
    }

    let mut score = 0;

    if has_lower {
        score += 1;
    }

    if has_upper {
        score += 1;
    }

    if has_number {
        score += 1;
    }

    if has_symbol {
        score += 1;
    }

    if password.len() >= 8 {
        score += 1;
    }

    if password.len() >= 12 {
        score += 1;
    }

    let strength = match score {
        0..=2 => "Weak",
        3..=4 => "Medium",
        _ => "Strong",
    };

    println!("\nPassword Analysis");
    println!("-----------------");
    println!("Password: {}", password);
    println!("Score: {}/6", score);
    println!("{} Lowercase letters", if has_lower { "✓" } else { "✗" });
    println!("{} Uppercase letters", if has_upper { "✓" } else { "✗" });
    println!("{} Numbers", if has_number { "✓" } else { "✗" });
    println!("{} Symbols", if has_symbol { "✓" } else { "✗" });
    println!("{} At least 8 characters", if password.len() >= 8 { "✓" } else { "✗" });
    println!("{} At least 12 characters", if password.len() >= 12 { "✓" } else { "✗" });
    println!("\nStrength: {}", strength);
}
