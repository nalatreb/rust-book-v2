use std::{io, process::exit};

fn main() {
    println!("What fibonacci number want to know? Please enter your number: ");

    let mut nth_number = String::new();

    io::stdin()
        .read_line(&mut nth_number)
        .expect("Unable to read input");

    let nth_number = match nth_number.trim().parse() {
        Ok(num) => num,
        Err(_) => exit(1),
    };

    // let fibonacci_number = fibonacci_recursive(nth_number);
    // println!("The {nth_number} fibonacci number is: {fibonacci_number}");

    let fibonacci_number = fibonacci(nth_number);
    println!("The {nth_number} fibonacci number is: {fibonacci_number}");
}

fn fibonacci_recursive(n: i64) -> i64 {
    match n {
        1 => 0,
        2 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

fn fibonacci(mut n: i64) -> i64 {
    match n {
        1 => 0,
        2 => 1,
        _ => {
            if n == 2 {
                return 1;
            }

            let mut result = 1;
            let mut n1: i64 = 1;
            let mut n2 = 0;

            while n != 2 {
                result = n1 + n2;
                n2 = n1;
                n1 = result;
                n = n - 1;
            }

            result
        }
    }
}
