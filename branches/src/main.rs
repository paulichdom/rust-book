fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // num_three();
    // is_divisible();

    let condition = true;
    let number = if condition {5} else {6};

    println!("The value of number is: {number}");

    calc_y();
}

fn num_three() {
    let number = 3;

    if number != 0 {
        println!("number was three");
    }
}

fn is_divisible() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

/*fn calc_y() {
    let x = 1;
    let y = if x { 0 } else { 1 };
    println!("{y}");
}*/

