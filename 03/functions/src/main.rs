fn five() -> i32 {
    5
}

fn plus_one(i: i32) -> i32 {
    i + 1
}

fn main() {
    let f = five();
    let i = plus_one(100);
    println!("The value of five is: {}", f);
    println!("The value of i is {}", i);
    another_fuction(5, 6);
}

fn another_fuction(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
