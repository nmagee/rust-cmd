use std::env;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if we have the correct number of arguments
    if args.len() != 4 {
        println!("Please provide exactly THREE numbers as arguments");
        println!("Usage: {} <number1> <number2> <number3>", args[0]);
        return;
    }

    // Try to parse each argument into a number
    let num1: f64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: {} is not a valid number", args[1]);
            return;
        }
    };

    let num2: f64 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: {} is not a valid number", args[2]);
            return;
        }
    };

    let num3: f64 = match args[3].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: {} is not a valid number", args[3]);
            return;
        }
    };

    // Calculate and print the sum
    let sum = num1 + num2 + num3;
    println!("The sum of {} + {} + {} = {}", num1, num2, num3, sum);
}
