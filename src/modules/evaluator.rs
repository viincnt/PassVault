use std::io;

pub fn evaluate_password() {
    loop {
        println!("\n\n\n");
        println!("Evaluating the strength of a password...");
        let mut password_input = String::new();
        io::stdin().read_line(&mut password_input).expect("Failed to read line");
        let password = password_input.trim();

        if password.is_empty() {
            println!("Input cannot be empty.");
            continue;
        }

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
        println!("\n\n\n");
        println!("Evaluated password: {}", password);
        println!("Password strength: {}", strength);
        println!("\n\n\n");
        break;
    }
}
