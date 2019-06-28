fn eighteen_one() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("using your favorite color {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn eighteen_two() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn eighteen_three() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn eighteen_four() {
    let (x, y, z) = (1, 2, 3);
    println!("{}{}{}", x, y, z);
}

// fn eighteen_five() {
//     let (x, y) = (1, 2, 3);
// }

fn eighteen_six() {
    fn foo(x: i32) {
        println!("{}", x);
    }
    foo(1);
}

fn eighteen_seven() {
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x,y);
    }

    let point = (3, 5);
    print_coordinates(&point);
}

// fn eighteen_eight() {
//     let some_option_value = Some(String::from("yes"));
//     let Some(x) = some_option_value;
// }

fn eighteen_nine() {
    let some_option_value = Some(String::from("yes"));

    if let Some(x) = some_option_value {
        println!("{}", x);
    }
}

fn eighteen_ten() { // compiles but shows warning `warning: irrefutable if-let pattern`
    if let x = 5 {
        println!("{}", x);
    }
}

fn eighteen_eleven() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn eighteen_twelve() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

fn eighteen_thirteen() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

fn eighteen_fourteen() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

fn eighteen_fifteen() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // let msg = Message::ChangeColor(0, 160, 255);
    // let msg = Message::Quit;
    // let msg = Message::Write(String::from("hey"));
    let msg = Message::Move{ x: 50, y: 3409 };

    match msg {
        Message::Quit => {
            println!("The Quit varient has no data to destructure");
        },
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        },
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
    }
}

fn eighteen_sixteen() {
    struct Point {
        x: i32,
        y: i32,
    }

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
}

fn eighteen_seventeen() {
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);
}

fn eighteen_eighteen() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Cant overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}",  setting_value);
}

fn eighteen_nineteen() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        },
    }
}

fn eighteen_twenty() {
    let _x = 5;
    let y = 10;
}

// fn eighteen_twentyone() {
//     let s = Some(String::from("Hello!"));
//
//     if let Some(_s) = s {
//         println!("found a string");
//     }
//     println!("{:?}", s);
// }

fn eighteen_twentytwo() {
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);
}

fn eighteen_twentythree() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}

fn eighteen_twentyfour() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

// fn eighteen_twentyfive() { // error: `..` can only be used once per tuple or tuple struct pattern
//     let numbers = (2, 4, 8, 16, 32);
//
//     match numbers {
//         (.., second, ..) => {
//             println!("Some numbers: {}", second)
//         },
//     }
// }

// fn eighteen_twentysix() { //borrow of moved value: `robot_name`
//     let robot_name = Some(String::from("Bors"));
//
//     match robot_name {
//         Some(name) => println!("Found a name: {}", name),
//         None => (),
//     }
//
//     println!("robot_name is: {:?}", robot_name);
// }

fn eighteen_twentyseven() {
    let robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);
}

fn eighteen_twentyeight() {
    let mut robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);
}

fn eighteen_twentynine() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

fn eighteen_thirty() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Match, n = {:?}", n),
        _ => println!("Default case, x {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn eighteen_thirtyone() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

fn eighteen_thirtytwo(){
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_variable @ 3...7 } => {
            println!("Found an ID in range: {}", id_variable)
        },
        Message::Hello { id: 10...12 } => {
            println!("Found an ID in another range")
        },
        Message::Hello { id } => {
            println!("Found another ID: {}", id);
        },
    }
}

fn main() {
    eighteen_one();
    eighteen_two();
    eighteen_three();
    eighteen_four();
    // eighteen_five();
    eighteen_six();
    eighteen_seven();
    // eighteen_eight();
    eighteen_nine();
    eighteen_ten();
    eighteen_eleven();
    eighteen_twelve();
    eighteen_thirteen();
    eighteen_fourteen();
    eighteen_fifteen();
    eighteen_sixteen();
    eighteen_seventeen();
    eighteen_eighteen();
    eighteen_nineteen();
    eighteen_twenty();
    // eighteen_twentyone();
    eighteen_twentytwo();
    eighteen_twentythree();
    eighteen_twentyfour();
    // eighteen_twentyfive();
    // eighteen_twentysix();
    eighteen_twentyseven();
    eighteen_twentyeight();
    eighteen_twentynine();
    eighteen_thirty();
    eighteen_thirtyone();
    eighteen_thirtytwo();
}
