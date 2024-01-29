fn main() {
    // let x = true;
    // read(x);

    let n = 5;
    let y = plus_one(n);
    println!("The value of y is: {y}");

    let a = 5;
    let mut b = a;
    b += 1;

    let a = Box::new([0; 1_000_000]);
    let b = a;
}

/*fn read(y: bool) {
    if y {
        println!("y is true");
    }
}*/

fn plus_one(x: i32) -> i32 {
 x + 1
}
