extern crate add_one;
extern crate rand;

fn main() {
    let num = 10;
    println!("Hello, World! {} plus one is {}!", num, add_one::add_one(num));
}