fn main() {
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    // println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // println!("Accessed values -> {five_hundred}, {six_point_four} and {one}");

    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    let _b = [3,5];

    // Q1: 
    /* let message = "The temperature today is:";
    let x = [message, 100];
    println!("{} {}", x[0], x[1]); */
    
    // Q2:
    let t = ([1; 2], [3; 4]);
    let (a, b) = t;
    println!("{}", a[0] + t.1[0]); 
}
