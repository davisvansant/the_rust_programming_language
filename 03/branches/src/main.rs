fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let num_bool = 10;

    // ^^^^^^^^ expected bool, found integer

    if num_bool != 0 {
        println!("something isnt right");
    }

    multiple_conditions();
    if_let()
}

fn multiple_conditions() {
    let number = 11;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3,2");
    }
}

fn if_let () {
    let condition = false;

    let number: &str = if condition {
        "yep!!"
    } else {
        "nope!!"
    }; //playing and understanding different options here

    println!("What is this condition ----- {}", number);
}
