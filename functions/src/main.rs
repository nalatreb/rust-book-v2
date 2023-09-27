fn main() {
    println!("Hello, world!");
    // another_function();
    another_function(5);

    print_labeled_measurement(5, 'h');

    // statement
    // 6: expression, which evaluates to 6
    let y = 6;

    // let x = (let y = 6); !!! YOU CANT DO THIS

    // if you have a semicolon at the end it will be a statement
    // without a semicolon it is an expression
    let z = {
        let x = 3;
        x + 1
    };

    let five = five();
    println!("The value of five is: {five}");

    let plus_one = plus_one(5);
    println!("The value of plus_one is: {plus_one}");
}

// fn another_function() {
//     println!("Another function.");
// }

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}