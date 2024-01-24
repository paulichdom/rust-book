fn main() {
    print_labeled_measurement(5, 'h');
    f(0);

    let y = {
        let x = 3;
        x + 1 // No semicolon. Adding a semicolon turns it into a statement, and it will then not return a value
    };

    println!("The value of y is: {y}");
    
    let x = five();
    println!("The value of x is: {x}");
    
    let p_one = plus_one(5);
    println!("The value of p_one is: {p_one}");

    println!("{}", test({

        let y = 1;
    
        y + 1
    
      }));
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn f(x: i32) { 
    println!("{x}");
  }

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn test(x: i32) -> i32 { x + 1 }

