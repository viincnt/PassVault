use std::io;
mod modules;

fn read_u32() -> Option<u32> {
    let mut menu_input = String::new();

    io::stdin().read_line(&mut menu_input).ok()?;

    menu_input.trim().parse().ok()
}

fn main() {
    loop {
        println!();
        println!("Password Toolkit");
        println!("----------------");
        println!("1. Generate a new password");
        println!("2. Strengthen an existing password");
        println!("3. Evaluate a password");
        println!("4. Exit");

        let menu_choice = match read_u32() {
            Some(n) => n,
            None => {
                println!("Invalid input.");
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
                println!("Invalid option.");
            }
        }
    }
}
