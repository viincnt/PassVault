use std::io;

pub fn strengthen_password() {
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
}
