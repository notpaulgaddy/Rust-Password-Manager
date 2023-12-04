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
                    let mut appName = String::new();
                    println("What is the name of the application you'd like to update?")
                    appName = appName.trim().to_string();
                    io::stdin()
                        .read_line(&mut app_name)
                        .expect("Failed to read line");
                    if my_map.contains_key(&appName){
                        println("Would you like to update the username or password for {}?", appName);
                        let mut choice = String::new();
                        choice = choice.trim().to_string();
                        io::stdin()
                            .read_line(&mut new_username)
                            .expect("Failed to read line");
                        match choice {
                            "username" => {
                                let mut newUsername = String:new();
                                io::stdin()
                                    .read_line(&mut newUsername)
                                    .expect("Failed to read line");
                                if let Some(entry) = my_map.get_mut(&appName){
                                    entry.0 = newUsername.trim().to_string();
                                    println("Username updated successfully")
                                }else{
                                    println("Failed to update username for {}", appName)
                                }
                            },
                            "password" => {
                                let mut newPassword = String:new();
                                io::stdin()
                                    .read_line(&mut newPassword)
                                    .expect("Failed to read line");
                                if let Some(entry) = my_map.get_mut(&appName){
                                    entry.0 = newPassword.trim().to_string();
                                    println("Password updated successfully")
                                }else{
                                    println("Failed to update password for {}", appName)
                                }
                            }
                        }
                    } else{
                        println("Application {} not found in the password manager.", appName)
                    }
                }
            }
        }
    } else {
        println!("Goodbye!");
    }
}