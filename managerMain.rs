use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::Write;
use std::env;
use std::fs;

fn main() {
    let mut start = String::new();
    let mut my_map = HashMap::new();
    println!("Would you like to use the password manager? (yes or no)");
    io::stdin()
        .read_line(&mut start)
        .expect("Failed to read line");
    
    start = start.trim().to_lowercase().to_string();

    if start == "yes" {
        let mut input = String::new();

        while input.trim() != "end" {
            println!("Enter a command (add, view, update, delete, end):");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input = input.trim().to_lowercase();

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

                    my_map.insert(
                        app_name.trim().to_string(),
                        (username.trim().to_string(), password.trim().to_string()),
                    );
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
                "update" => {
                    println!("What is the name of the application you'd like to update?");
                    let mut app_name = String::new();
                    io::stdin()
                        .read_line(&mut app_name)
                        .expect("Failed to read line");
                    let app_name = app_name.trim().to_string();

                    if my_map.contains_key(&app_name) {
                        println!(
                            "Would you like to update the username or password for {}?",
                            app_name
                        );
                        let mut choice = String::new();
                        io::stdin()
                            .read_line(&mut choice)
                            .expect("Failed to read line");
                        let choice = choice.trim().to_lowercase();

                        match choice.as_str() {
                            "username" => {
                                println!("Enter the new username:");
                                let mut new_username = String::new();
                                io::stdin()
                                    .read_line(&mut new_username)
                                    .expect("Failed to read line");

                                if let Some(entry) = my_map.get_mut(&app_name) {
                                    entry.0 = new_username.trim().to_string();
                                    println!("Username updated successfully");
                                } else {
                                    println!("Failed to update username for {}", app_name);
                                }
                            }
                            "password" => {
                                println!("Enter the new password:");
                                let mut new_password = String::new();
                                io::stdin()
                                    .read_line(&mut new_password)
                                    .expect("Failed to read line");

                                if let Some(entry) = my_map.get_mut(&app_name) {
                                    entry.1 = new_password.trim().to_string();
                                    println!("Password updated successfully");
                                } else {
                                    println!("Failed to update password for {}", app_name);
                                }
                            }
                            _ => {
                                println!("Unknown choice: {}", choice);
                            }
                        }
                    } else {
                        println!("Application {} not found in the password manager.", app_name);
                    }
                }
                "delete" => {
                    println!("What is the name of the application you'd like to delete?");
                    let mut app_name = String::new();
                    io::stdin()
                        .read_line(&mut app_name)
                        .expect("Failed to read line");
                    let app_name = app_name.trim().to_lowercase();

                    if my_map.contains_key(&app_name) {
                        println!("Are you sure you want to delete your saved info for this application? (yes or no)");
                        let mut choice = String::new();
                        io::stdin()
                            .read_line(&mut choice)
                            .expect("Failed to read line");
                        let choice = choice.trim().to_lowercase();

                        match choice.as_str() {
                            "no" => {
                                // Do nothing, exit the "delete" case.
                            }
                            "yes" => {
                                if my_map.contains_key(&app_name) {
                                    my_map.remove(&app_name);
                                    println!("Application {} deleted successfully", app_name);
                                }
                            }
                            _ => {
                                println!("Invalid choice: {}", choice);
                            }
                        }
                    } else {
                        println!("Application {} not found in the password manager.", app_name);
                    }
                }
                "save" => {
                    let mut data_file = File::create("accounts.txt").expect("creation failed");
                    //format will be Company Name: ["Email": email@gmail.com, "Password": password123]
                    for (company_name, (email, password)) in &my_map {
                        let mut output = String::new();
                        output.push_str(company_name);
                        output.push(' '); // Add a space
                        output.push_str(email);
                        output.push(' '); // Add a space
                        output.push_str(password);
                        output.push_str("\n");
                        data_file.write(output.as_bytes()).expect("write failed");
                    }
                }
                "import" => {
                    let mut import_file_name = String::new();
                    println!("Enter the name of the text file to import:");
                    io::stdin()
                        .read_line(&mut import_file_name)
                        .expect("Failed to read line");
        
                    // Read through the txt file
                    let file_contents = fs::read_to_string(import_file_name.trim())
                        .expect("Failed to read file");
        
                    // Split file contents by lines and process each line
                    for line in file_contents.lines() {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() == 3 {
                            let company_name = parts[0];
                            let email = parts[1];
                            let password = parts[2];
                            txtPasswords.insert(company_name.to_string(), (email.to_string(), password.to_string()));
                        } else {
                            println!("Invalid line format: {}", line);
                        }
                    }
                    println!("Data imported successfully.");
                }
                _ => {
                    println!("Unknown command: {}", input);
                }
            }
        }
    } else {
        println!("Goodbye!");
    }
}
