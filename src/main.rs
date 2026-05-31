use std::io;
mod modules;

fn main() {
    println!("Choose an option:");
    println!("1. Generate a new password");
    println!("2. Strengthen an existing password");
    println!("3. Evaluate the strength of a password");
    println!("4. Exit");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let choice: u32 = input.trim().parse().expect("Please enter a number");

    match choice {
        1 => modules::generator::generate_password(),
        2 => modules::strengthener::strengthen_password(),
        3 => modules::evaluator::evaluate_password(),
        4 => println!("Exiting..."),
        _ => println!("Invalid option. Please try again."),
    }
}
