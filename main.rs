use std::collections::HashMap;
use std::io;

fn main() {
    let mut start = String::new();
    let mut my_map = HashMap::new();

    println!("Would you like to use the password manager? (yes or no)");
    io::stdin()
        .read_line(&mut start)
        .expect("Failed to read line");

    start = start.trim().to_lowercase();

    if start == "yes" {
        let mut input = String::new();

        while input.trim() != "end" {
            println!("Enter a command (add, end):");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            input = input.trim().to_lowercase();

            match input.as_str() {
                "add" => {
                    println!("What is the name of the application?");
                    let mut app_name = String::new();
                    io::stdin()
                        .read_line(&mut app_name)
                        .expect("Failed to read line");

                    println!("What is your username?");
                    let mut username = String::new();
                    io::stdin()
                        .read_line(&mut username)
                        .expect("Failed to read line");

                    println!("What is your password?");
                    let mut password = String::new();
                    io::stdin()
                        .read_line(&mut password)
                        .expect("Failed to read line");

                    my_map.insert(app_name.trim().to_string(), (username.trim().to_string(), password.trim().to_string()));
                }
                "view" => {
                    for (app_name, (username, password)) in &my_map {
                        println!("Application: {}", app_name);
                        println!("Username: {}", username);
                        println!("Password: {}", password);
                        println!();
                    }
                }
                "end" => {
                    println!("Goodbye!");
                    break;
                }
                _ => {
                    println!("Unknown command: {}", input);
                }
                "update" => {
                    
                }
            }
        }
    } else {
        println!("Goodbye!");
    }
}
