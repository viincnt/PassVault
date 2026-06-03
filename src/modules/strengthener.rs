use rand::seq::SliceRandom;
use rand::Rng;

pub fn strengthen(password: &str) -> String {
    let charset: Vec<char> =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*".chars().collect();

    let mut rng = rand::thread_rng();
    let mut strengthened = password.to_string();

    while strengthened.len() < 8 {
        strengthened.push(charset[rng.gen_range(0..charset.len())]);
    }

    for _ in 0..4 {
        strengthened.push(charset[rng.gen_range(0..charset.len())]);
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
    chars.into_iter().collect()
}
