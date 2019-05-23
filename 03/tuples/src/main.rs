fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("x.0 is {}", five_hundred);
    println!("x.1 is {}", six_point_four);
    println!("x.2 is {}", one);
}
