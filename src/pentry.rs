use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceInfo {
    pub service: String,
    pub username: String,
    pub password: String,
}

impl ServiceInfo {
    pub fn new(service: String, username: String, password: String) -> Self {
        ServiceInfo {
            service,
            username,
            password,
        }
    }

    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_string)
    }

    #[allow(dead_code)]
    pub fn from_user_input() -> Self {
        let service = prompt("Enter Service Name: ");
        let username = prompt("Enter Your Username: ");
        let password = prompt("Enter Your Password: ");

        ServiceInfo::new(service, username, password)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Failed to serialize to JSON")
    }

    pub fn write_to_file(&self) {
        let json_output = format!("{}\n", self.to_json());

        match OpenOptions::new().create(true).append(true).open("passwords.json") {
            Ok(mut file) => {
                if let Err(e) = file.write_all(json_output.as_bytes()) {
                    eprintln!("Error writing to file: {}", e);
                } else {
                    println!("Successfully wrote to passwords.json");
                }
            }
            Err(e) => eprintln!("Error opening file: {}", e),
        }
    }
}

pub fn read_passwords_from_file() -> Result<Vec<ServiceInfo>, io::Error> {
    let file = File::open("passwords.json")?;
    let reader = io::BufReader::new(file);
    let mut services: Vec<ServiceInfo> = Vec::new();

    for line in reader.lines() {
        if let Ok(json_string) = line {
            if let Ok(service_info) = ServiceInfo::from_json(&json_string) {
                services.push(service_info);
            }
        }
    }
    Ok(services)
}

pub fn delete_from_file() -> Result<(), io::Error> {
    println!("Please give me the name of the service you want to delete:");
    let service_name = prompt("Service Name: ");

    let mut services = read_passwords_from_file()?;
    let mut found = false;

    services.retain(|service| {
        if service.service == service_name {
            found = true;
            false // Exclude this service from the retained list
        } else {
            true // Keep this service in the list
        }
    });

    if found {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("passwords.json")?;

        for service in &services {
            let json_output = format!("{}\n", service.to_json());
            if let Err(e) = file.write_all(json_output.as_bytes()) {
                eprintln!("Error writing to file: {}", e);
            }
        }

        println!("Service '{}' deleted successfully.", service_name);
    } else {
        println!("Service '{}' not found in database.", service_name);
    }

    Ok(())
}


pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
