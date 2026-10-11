use std::collections::HashMap;
use std::io;

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Employee manager");
        println!("Commands:");
        println!("Add <name> to <group>");
        println!("List <group>");
        println!("List");
        println!("Quit");
        println!();
        // print!(" >> ");
        let mut user_command = String::new();

        io::stdin()
            .read_line(&mut user_command)
            .expect("Failed to read line");

        let parts: Vec<&str> = user_command.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }


        match parts[0].to_lowercase().as_str() {
            "add" if parts.len() == 4 && parts[2].eq_ignore_ascii_case("to") => {
                let name = parts[1].to_string();
                let group = parts[3].to_string();

                employees.entry(group).or_default().push(name);
            },
            "list" if parts.len() == 2 => {
                let group = parts[1].to_string();

                println!("{:?}", employees.entry(group).or_default());
            }
            "list" if parts.len() == 1 => {
                for (group, emp) in employees.iter() {
                    println!("{}: {:?}", group, emp);
                }
            }
            "quit" => break,
            _ => println!("Invalid command."),
        }

        println!();
    }

}
