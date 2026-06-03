pub struct EvaluationResult {
    pub score: u8,
    pub has_lower: bool,
    pub has_upper: bool,
    pub has_number: bool,
    pub has_symbol: bool,
    pub length: usize,
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

    EvaluationResult {
        score,
        has_lower,
        has_upper,
        has_number,
        has_symbol,
        length: password.len(),
    }
}
