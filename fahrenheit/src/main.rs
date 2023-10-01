use std::{io, process::exit};

fn main() {
    loop {
        println!("Please choose one:");
        println!("1 - Celsius to Fahrenheit");
        println!("2 - Farhenheit to Celsius");
        println!("3 - Exit");

        let conversion_type = match read_int_from_console() {
            Some(num) => num,
            None => continue,
        };

        if conversion_type == 3 {
            exit(0);
        }

        println!("Enter the value for converting: ");

        let value = match read_int_from_console() {
            Some(num) => num,
            None => continue,
        };

        if conversion_type == 1 {
            let result = convert_to_fahrenheit(value);
            println!("The temperature in Fahrenheit: {result}")
        }

        if conversion_type == 2 {
            let result = convert_to_celsius(value);
            println!("The temperature in Celsius: {result}")
        }
    }
}

fn read_int_from_console() -> Option<i32> {
    let mut selected = String::new();

    io::stdin()
        .read_line(&mut selected)
        .expect("Failed to read line");

    return match selected.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    };
}

fn convert_to_fahrenheit(value: i32) -> f64 {
    ((value as f64) * 9.0 / 5.0) + 32.0
}

fn convert_to_celsius(value: i32) -> f64 {
    ((value as f64) - 32.0) * 5.0 / 9.0
}
