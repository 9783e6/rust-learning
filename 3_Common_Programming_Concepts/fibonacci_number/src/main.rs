use std::io;

fn get_fibonacci_number(n: u32) -> u32 {
    let mut first_number = 0;
    let mut second_number = 1;
    for _ in (1..n-1) {
        let temp = first_number + second_number;
        first_number = second_number;
        second_number = temp;
    }
    second_number
}

fn main() {
    println!("Enter N:");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line.");
    let user_input: u32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let number = get_fibonacci_number(user_input);
    println!("Fibonacci number #{user_input} = {number}.")
}

