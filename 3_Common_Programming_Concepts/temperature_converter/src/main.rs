use std::io;

fn main() {
    loop {
        println!("C - F Converter");
        println!("1 - To convert celsius to fahrenheit");
        println!("2 - To convert fahrenheit to celsius");
        println!("0 - To exit");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line.");
        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if user_input == 1 {
            println!("Converting celsius to fahrenheit:");
            println!("Enter tempreture in celsius:");
            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line.");
            let user_input: f64 = match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            let fahren = (user_input*9.0/5.0)+32.0;
            println!("{user_input}C = {fahren}F");
        } else if user_input == 2 {
            println!("Converting fahrenheit to celsius:");
            println!("Enter tempreture in fahrenheit:");
            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read line.");
            let user_input: f64 = match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            let celsius = (user_input-32.0)*5.0/9.0;
            println!("{user_input}F = {celsius}C");
        } else if user_input == 0 {
            break
        } else {
            continue
        }
    }
}
