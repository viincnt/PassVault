use std::io;
use rand::Rng;
use rand::seq::SliceRandom;

pub fn strengthen_password() {
    loop {
        println!("\n\n\n");
        println!("Strengthening an existing password...");

        let mut password_input = String::new();
        io::stdin().read_line(&mut password_input).expect("Failed to read line");
        let password = password_input.trim();

        if password.is_empty() {
            println!("Input cannot be empty.");
            continue;
        }

        let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*";

        let mut strengthened = password.to_string();
        for _ in 0..4 {
            let idx = rand::thread_rng().gen_range(0..charset.len());
            strengthened.push(charset.chars().nth(idx).unwrap());
        }

        if password.len() < 8 {
            for _ in 0..8 - password.len() {
                let idx = rand::thread_rng().gen_range(0..charset.len());
                strengthened.push(charset.chars().nth(idx).unwrap());
            }
        }

        let mut chars: Vec<char> = strengthened.chars().collect();
        let uppercase_count = rand::thread_rng().gen_range(1..=chars.len());
        for _ in 0..uppercase_count {
            let idx = rand::thread_rng().gen_range(0..chars.len());
            chars[idx] = chars[idx].to_ascii_uppercase();
        }
        chars.shuffle(&mut rand::thread_rng());
        let strengthened: String = chars.into_iter().collect();
        println!("\n\n\n");
        println!("Strengthened password: {}", strengthened);
        println!("\n\n\n");
        break;
    }
}
