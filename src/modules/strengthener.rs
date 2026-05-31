use std::io;
use rand::Rng;
use rand::seq::SliceRandom;

pub fn strengthen_password() {
    println!("Strengthening an existing password...");

    let password = loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if !input.is_empty() {
            break input.to_string();
        }

        println!("Input cannot be empty.");
    };

    let charset: Vec<char> =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*".chars().collect();

    let mut rng = rand::thread_rng();

    let mut strengthened = password;

    while strengthened.len() < 8 {
        let random_char = charset[rng.gen_range(0..charset.len())];
        strengthened.push(random_char);
    }

    for _ in 0..4 {
        let random_char = charset[rng.gen_range(0..charset.len())];
        strengthened.push(random_char);
    }

    let mut chars: Vec<char> = strengthened.chars().collect();

    let letter_indices: Vec<usize> = chars
        .iter()
        .enumerate()
        .filter(|(_, c)| c.is_ascii_alphabetic())
        .map(|(i, _)| i)
        .collect();

    if !letter_indices.is_empty() {
        let uppercase_count = rng.gen_range(1..=letter_indices.len());

        for &idx in letter_indices.choose_multiple(&mut rng, uppercase_count) {
            chars[idx] = chars[idx].to_ascii_uppercase();
        }
    }

    chars.shuffle(&mut rng);

    let strengthened: String = chars.into_iter().collect();

    println!("\nStrengthened password: {}", strengthened);
}
