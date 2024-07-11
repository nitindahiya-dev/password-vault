mod pentry;

use crate::pentry::{prompt, read_passwords_from_file, delete_from_file, update_entry, ServiceInfo};

fn clr() {
    // This escape code clears the terminal screen
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    clr();

    let ascii = r#"
    
░▒▓███████▓▒░ ░▒▓██████▓▒░ ░▒▓███████▓▒░▒▓███████▓▒░      ░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░   ░▒▓████████▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░     ░▒▓█▓▒░             ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░     
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░     ░▒▓█▓▒░              ░▒▓█▓▒▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░     
░▒▓███████▓▒░░▒▓████████▓▒░░▒▓██████▓▒░░▒▓██████▓▒░        ░▒▓█▓▒▒▓█▓▒░░▒▓████████▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░     
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░     ░▒▓█▓▒░        ░▒▓█▓▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░     
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░      ░▒▓█▓▒░     ░▒▓█▓▒░        ░▒▓█▓▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░     
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓███████▓▒░▒▓███████▓▒░          ░▒▓██▓▒░  ░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░░▒▓████████▓▒░▒▓█▓▒░     
    "#;

    println!("{}", ascii);

    loop {
        println!("Password manager menu:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search Entry");
        println!("4. Delete");
        println!("5. Update Entry");
        println!("6. Exit Now");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service: "),
                    prompt("Username: "),
                    prompt("Password: "),
                );

                println!("Entry added successfully.");
                entry.write_to_file();
            }

            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });

                for item in &services {
                    println!(
                        "Service: {}\n- Username: {}\n- Password: {}",
                        item.service, item.username, item.password
                    );
                }
            }

            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}", err);
                    Vec::new()
                });

                let search = prompt("Search: ");
                let mut found: bool = false;
                for item in &services {
                    if item.service.as_str() == search.as_str() {
                        println!(
                            "Service: {}\n- Username: {}\n- Password: {}",
                            item.service, item.username, item.password
                        );
                        found = true;
                        break;
                    }
                }
                if !found {
                    println!("This service is not in our database..Please try again.");
                }
            }

            "4" => {
                clr();
                match delete_from_file() {
                    Ok(_) => println!("Service deleted successfully."),
                    Err(e) => eprintln!("Error deleting service: {}", e),
                }
            }

            "5" => {
                clr();
                match update_entry() {
                    Ok(_) => println!("Service updated successfully."),
                    Err(e) => eprintln!("Error updating service: {}", e),
                }
            }

            "6" => {
                clr();
                println!("Feel Free to Try Again Later");
                break;
            }

            _ => println!("Please Use Option 1, 2, 3, 4, 5, or 6."),
        }

        println!("\n\n");
    }
}
