use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
}

fn add(name: &str) {
    let contents: Result<String, std::io::Error> = fs::read_to_string("user.json");
    let mut user_array: Vec<User> = match contents {
        Ok(content) => serde_json::from_str(&content).expect("Failed to deserialize"),
        Err(_error) => Vec::new(),
    };
    let new_user = User { name: name.to_string().clone() };
    user_array.push(new_user);
    let serialized_users = serde_json::to_string(&user_array).expect("Failed to serialize");
    fs::write("user.json", serialized_users).expect("Failed to write to file");
    println!("User added in DB: {name}");
}

fn update(current_value: &str, new_value: &str) {
    let contents: Result<String, std::io::Error> = fs::read_to_string("user.json");
    let mut user_array: Vec<User> = match contents {
        Ok(content) => serde_json::from_str(&content).expect("Failed to deserialize"),
        Err(_error) => Vec::new(),
    };
    if let Some(index) = user_array.iter().position(|u| u.name == current_value) {
        user_array[index].name = new_value.to_string().clone();
        let serialized_users = serde_json::to_string(&user_array).expect("Failed to serialize");
        fs::write("user.json", serialized_users).expect("Failed to write to file");
        println!("User name updated in DB : from {current_value} to {new_value}");
    } else {
        println!("Unable to find user with given name {current_value}");
    }
}

fn display() {
    let contents: Result<String, std::io::Error> = fs::read_to_string("user.json");
    let user_array: Vec<User> = match contents {
        Ok(content) => serde_json::from_str(&content).expect("Failed to deserialize"),
        Err(_error) => Vec::new(),
    };
    for user in user_array {
        println!("{}", user.name);
    }
}

fn main() {
    loop {
        print!("Enter Command: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input_array: Vec<&str> = input.split("-").collect();

        if input_array.is_empty() {
            println!("No input provided");
        } else {
            let command = input_array[0].trim();
            match command {
                "exit" => break,
                "display" => display(),
                "add" => {
                    if input_array.len() < 2 {
                        println!("Not enough arguments for 'add' command");
                    } else {
                        add(input_array[1].trim());
                    }
                }
                "update" => {
                    if input_array.len() < 3 {
                        println!("Not enough arguments for 'add' command");
                    } else {
                        update(input_array[1].trim(), input_array[2].trim());
                    }
                }
                _ => println!("Invalid Input"),
            }
        }
    }
}
