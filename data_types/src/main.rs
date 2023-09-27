use std::io;

fn main() {
    // let guess: u32 = "42".parse().expect("Not a number!");
    
    // let x = 2.0;
    // let y: f32 = 3.0;

    // let sum = 5 + 10;

    // let difference = 95.5 - 4.3;

    // let product = 4 * 30;

    // let quotent = 56.7 / 32.2;
    // let truncated = -5 / 3;

    // let remainder = 43 % 5;

    // let t = true;
    // let f: bool = false;

    // let c = 'z';
    // let z: char = 'ℤ';
    // let heart_eyed_cat = '😻';

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (a, b, c) = tup;

    // println!("The values of a are: {a}");

    // let d: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = d.0;
    // let six_point_four = d.1;
    // let one = d.2;

    // let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    // let a: [i32; 5] = [1, 2, 3, 4, 5];

    // let a = [3; 5]; = let a = [3, 3, 3, 3, 3];

    // let a = [1, 2, 3, 4, 5];
    // let first = a[0];
    // let second = a[1];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
