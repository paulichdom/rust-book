fn main() {
    print_labeled_measurement(5, 'h');
    f(0);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn f(x) { 
    println!("{x}");
  }
