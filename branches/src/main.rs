fn main() {
    let number = 6;

    if number < 5 {
        println!("contition was true");
    } else {
        println!("condition was false");
    }

    // if number {
    //     println!("number was seven");
    // }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number2 = if condition { 5 } else { 6 };

    // YOU CANT DO THIS
    // let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number2}");
}
