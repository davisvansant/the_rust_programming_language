fn main() {
    //this_is_a_loop();
    //this_is_a_while();
    this_is_a_for();
}

fn this_is_a_loop() {
    loop {
        println!("weeeeee");
    }
}

fn this_is_a_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

fn this_is_a_for() {
    let stuff = [10, 20, 30, 40, 50];

    for thingy in stuff.iter() {
        println!("the value is {}", thingy);
    }

    for number in (1..10).rev() {
        println!("{}!", number);
    }
}
