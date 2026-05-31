use std::io;
use rand::Rng;

fn generate_weak() -> String {
    let charset = "abcdefghijklmnopqrstuvwxyz";
    (0..8)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

fn generate_medium() -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    (0..12)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

fn generate_strong() -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*";
    (0..16)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect()
}

pub fn generate_password() {
    loop {
        println!("\n\n\n");
        println!("Select the desired password strength:");
        println!("1. Weak");
        println!("2. Medium");
        println!("3. Strong");

        let mut strength_input = String::new();
        io::stdin().read_line(&mut strength_input).expect("Failed to read line");

        let strength_choice = match strength_input.trim().parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                print!("Invalid input. Please enter a number between 1 and 3.\n\n\n");
                continue;
            }
        };
        println!("\n\n\n");
        match strength_choice {
            1 => println!("Generated password: {}", generate_weak()),
            2 => println!("Generated password: {}", generate_medium()),
            3 => println!("Generated password: {}", generate_strong()),
            _ => {
                print!("Invalid choice. Please enter a number between 1 and 3.\n\n\n");
                continue;
            }
        }
        println!("\n\n\n");
        break;
    }
}
