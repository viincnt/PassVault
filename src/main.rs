use std::io;
mod modules;

fn main() {
    loop {
        println!("Choose an option:");
        println!("1. Generate a new password");
        println!("2. Strengthen an existing password");
        println!("3. Evaluate the strength of a password");
        println!("4. Exit");

        let mut menu_input = String::new();
        io::stdin().read_line(&mut menu_input).expect("Failed to read line");
        let menu_choice = match menu_input.trim().parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                print!("Invalid input. Please enter a number between 1 and 4.\n\n\n");
                continue;
            }
        };

        match menu_choice {
            1 => modules::generator::generate_password(),
            2 => modules::strengthener::strengthen_password(),
            3 => modules::evaluator::evaluate_password(),
            4 => {
                println!("Exiting...");
                break;
            }
            _ => {
                print!("Invalid choice. Please enter a number between 1 and 4.\n\n\n");
            }
        }
    }
}
