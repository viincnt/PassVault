use rand::seq::{ IteratorRandom, SliceRandom };
use rand::Rng;
use std::fs;

fn load_words() -> Vec<String> {
    let content = fs::read_to_string("src/assets/words.txt").expect("Failed to load words.txt");
    content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect()
}

fn mutate_word(word: &str) -> String {
    let mut rng = rand::thread_rng();
    word.chars()
        .map(|c| {
            let lower = c.to_ascii_lowercase();
            let mut result = match lower {
                'a' if rng.gen_bool(0.35) => '@',
                'b' if rng.gen_bool(0.2) => '8',
                'e' if rng.gen_bool(0.35) => '3',
                'g' if rng.gen_bool(0.2) => '9',
                'i' if rng.gen_bool(0.35) => '1',
                'l' if rng.gen_bool(0.2) => '|',
                'o' if rng.gen_bool(0.35) => '0',
                's' if rng.gen_bool(0.35) => '$',
                't' if rng.gen_bool(0.3) => '7',
                'z' if rng.gen_bool(0.2) => '2',
                _ => c,
            };
            if result.is_ascii_alphabetic() && rng.gen_bool(0.35) {
                result = result.to_ascii_uppercase();
            }
            result
        })
        .collect()
}

pub fn generate_memorable(word_count: usize, include_specials: bool) -> String {
    let words = load_words();
    let mut rng = rand::thread_rng();

    let selected: Vec<String> = words
        .iter()
        .choose_multiple(&mut rng, word_count)
        .into_iter()
        .map(|word| if include_specials { mutate_word(word) } else { word.clone() })
        .collect();

    let mut password = selected.join("-");

    if include_specials {
        let extras = ['!', '@', '#', '$', '%', '&', '*', '0', '1', '2', '3', '5', '7', '8', '9'];
        password.push(*extras.choose(&mut rng).unwrap());
    }

    password
}

pub fn generate_random(length: usize, include_specials: bool) -> String {
    let lowercase = "abcdefghijklmnopqrstuvwxyz";
    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numbers = "0123456789";
    let specials = "!@#$%^&*";

    let mut rng = rand::thread_rng();
    let mut charset = format!("{lowercase}{uppercase}{numbers}");
    if include_specials {
        charset.push_str(specials);
    }

    let charset: Vec<char> = charset.chars().collect();
    let mut password = Vec::with_capacity(length);

    password.push(
        lowercase
            .chars()
            .nth(rng.gen_range(0..lowercase.len()))
            .unwrap()
    );
    password.push(
        uppercase
            .chars()
            .nth(rng.gen_range(0..uppercase.len()))
            .unwrap()
    );
    password.push(
        numbers
            .chars()
            .nth(rng.gen_range(0..numbers.len()))
            .unwrap()
    );

    if include_specials {
        password.push(
            specials
                .chars()
                .nth(rng.gen_range(0..specials.len()))
                .unwrap()
        );
    }

    while password.len() < length {
        password.push(charset[rng.gen_range(0..charset.len())]);
    }

    password.shuffle(&mut rng);
    password.into_iter().collect()
}
