fn common_patterns() -> impl Iterator<Item = &'static str> {
    include_str!("../assets/common_patterns.txt")
        .lines()
        .filter(|l| !l.is_empty())
}

fn keyboard_seqs() -> impl Iterator<Item = &'static str> {
    include_str!("../assets/keyboard_seqs.txt")
        .lines()
        .filter(|l| !l.is_empty())
}

pub struct EvaluationResult {
    pub score: u8,
    pub has_lower: bool,
    pub has_upper: bool,
    pub has_number: bool,
    pub has_symbol: bool,
    pub length: usize,
    pub no_repeats: bool,
    pub no_sequences: bool,
    pub not_common: bool,
    pub very_long: bool,
}

pub fn evaluate(password: &str) -> EvaluationResult {
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

    let chars: Vec<char> = password.chars().collect();
    let lower = password.to_lowercase();

    let no_repeats = chars.len() < 3 || !chars.windows(3).any(|w| w[0] == w[1] && w[1] == w[2]);

    let has_numeric_or_alpha_run =
        chars.len() >= 3 &&
        chars.windows(3).any(|w| {
            let (a, b, c) = (w[0] as i32, w[1] as i32, w[2] as i32);
            (b - a == 1 && c - b == 1) || (a - b == 1 && b - c == 1)
        });
    let has_keyboard_run = keyboard_seqs().any(|seq| lower.contains(seq));
    let no_sequences = !has_numeric_or_alpha_run && !has_keyboard_run;

    let not_common = !common_patterns().any(|p| lower.contains(p));

    let very_long = password.len() >= 16;

    let mut score: u8 = 0;
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
    if no_repeats {
        score += 1;
    }
    if no_sequences {
        score += 1;
    }
    if not_common {
        score += 1;
    }
    if very_long {
        score += 1;
    }

    EvaluationResult {
        score,
        has_lower,
        has_upper,
        has_number,
        has_symbol,
        length: password.len(),
        no_repeats,
        no_sequences,
        not_common,
        very_long,
    }
}
