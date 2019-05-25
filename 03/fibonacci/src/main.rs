fn fibonacci(n: i64) -> i64 {
    if n == 0 {
        1
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n -2)
    }
}

fn main() {
    for i in 1..45 { //this is about the upper range my system can handle before topping out 
        println!("{:?}", fibonacci(i));
    }
}
