fn main() {
    // Aquabats - Attacked by Snakes! https://www.youtube.com/watch?v=XegiATQjYwg

    let snake = "\u{1F40D}".repeat(10);

    for n in (1..20).step_by(2) {
        println!("{:?} Snake, {:?} Snake", n, (n + 1));
        println!("Are there any more?");
        println!("Theres two more");
        println!("Knocking on my door...");
        println!("{}", snake);
    }
}
