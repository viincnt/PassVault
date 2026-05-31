use std::io;
use rand::Rng;

pub fn generate_password() {
    println!("Select the desired password strength:");
    println!("1. Weak");
    println!("2. Medium");
    println!("3. Strong");
    let mut strength_input = String::new();
    io::stdin().read_line(&mut strength_input).expect("Failed to read line");
    let strength_choice: u32 = strength_input.trim().parse().expect("Please enter a number");

    let weak_password_chars = "abcdefghijklmnopqrstuvwxyz";
    let medium_password_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
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
}
