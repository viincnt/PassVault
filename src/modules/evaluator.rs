use std::io;

pub fn evaluate_password() {
    println!("Evaluating the strength of a password...");
    let mut password_input = String::new();
    io::stdin().read_line(&mut password_input).expect("Failed to read line");
    let password = password_input.trim();

    let mut score = 0;

    if password.chars().any(|c| c.is_lowercase()) {
        score += 1;
    }

    if password.chars().any(|c| c.is_uppercase()) {
        score += 1;
    }

    if password.chars().any(|c| c.is_numeric()) {
        score += 1;
    }

    if password.chars().any(|c| !c.is_alphanumeric()) {
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

    println!("Evaluated password: {}", password);
    println!("Password strength: {}", strength);
}
