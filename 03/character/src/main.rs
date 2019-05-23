fn main() {
    let c = 'z';
    let z = '\u{017E}';
    let rust_is_awesome = "\u{1f980}".repeat(10);

    println!("c is {}", c);
    println!("z is {}", z);
    println!("{}", rust_is_awesome);
}
