use std::io;
use rand::Rng;

fn main() {
    println!("Choose an option:");
    println!("1. Generate a new password");
    println!("2. Strengthen an existing password");
    println!("3. Evaluate the strength of a password");
    println!("4. Exit");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let choice: u32 = input.trim().parse().expect("Please enter a number");

    let generate_password = || {
        println!("Select the desired password strength:");
        println!("1. Weak");
        println!("2. Medium");
        println!("3. Strong");
        let mut strength_input = String::new();
        io::stdin().read_line(&mut strength_input).expect("Failed to read line");
        let strength_choice: u32 = strength_input.trim().parse().expect("Please enter a number");

        let weak_password_chars = "abcdefghijklmnopqrstuvwxyz";
        let medium_password_chars =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let strong_password_chars =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:,.<>?";

        let generated_password = match strength_choice {
            1 =>
                (0..8)
                    .map(|_| {
                        let idx = rand::thread_rng().gen_range(0..weak_password_chars.len());
                        weak_password_chars.chars().nth(idx).unwrap()
                    })
                    .collect::<String>(),
            2 =>
                (0..12)
                    .map(|_| {
                        let idx = rand::thread_rng().gen_range(0..medium_password_chars.len());
                        medium_password_chars.chars().nth(idx).unwrap()
                    })
                    .collect::<String>(),
            3 =>
                (0..16)
                    .map(|_| {
                        let idx = rand::thread_rng().gen_range(0..strong_password_chars.len());
                        strong_password_chars.chars().nth(idx).unwrap()
                    })
                    .collect::<String>(),
            _ => "weak_password".to_string(),
        };

        println!("Generated password: {}", generated_password);
    };

    let strengthen_password = || {
        println!("Strengthening an existing password...");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let password = input.trim();

        let has_upper = password.chars().any(|c| c.is_uppercase());
        let has_digit = password.chars().any(|c| c.is_numeric());
        let has_special = password.chars().any(|c| "!@#$%&*".contains(c));

        let mut strengthened = password.to_string();

        if !has_upper {
            strengthened.push('A');
        }
        if !has_digit {
            strengthened.push('1');
        }
        if !has_special {
            strengthened.push('!');
        }

        println!("Strengthened password: {}", strengthened);
    };

    let evaluate_password = || {
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
    };

    match choice {
        1 => generate_password(),
        2 => strengthen_password(),
        3 => evaluate_password(),
        4 => println!("Exiting..."),
        _ => println!("Invalid option. Please try again."),
    }
}
