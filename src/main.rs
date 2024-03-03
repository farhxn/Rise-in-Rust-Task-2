enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b == 0.0 {
                println!("Cannot divide by zero.");
                0.0
            } else {
                a / b
            }
        },
    }
}
use std::io;

fn main() {
    let mut first_number_str = String::new();
    let mut operation_str = String::new();
    let mut second_number_str = String::new();

    println!("Enter the first number:");
    io::stdin().read_line(&mut first_number_str).expect("Failed to read line");
    let first_number: f64 = first_number_str.trim().parse().expect("Please type a number!");

    println!("Enter the operation (+, -, *, /):");
    io::stdin().read_line(&mut operation_str).expect("Failed to read line");
    let operation_char = operation_str.trim();

    println!("Enter the second number:");
    io::stdin().read_line(&mut second_number_str).expect("Failed to read line");
    let second_number: f64 = second_number_str.trim().parse().expect("Please type a number!");

    let operation = match operation_char {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    let result = calculate(operation);
    println!("The result is: {}", result);
}


