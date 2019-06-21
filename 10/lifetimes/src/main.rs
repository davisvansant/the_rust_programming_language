// fn ten_seventeen() {
//     let r;
//
//     {
//         let x = 5;
//         r = &x;
//     }
//
//     println!("r: {}", r);
// }

// fn ten_eighteen() {
//     let r;
//
//     {
//         let x = 5;
//         r = &x;
//     }
//
//     println!("r: {}", r);
// }

fn ten_nineteen() {
    let x = 5;
    let r = &x;

    println!("r: {}", r);
}

// fn ten_twenty() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";
//
//     let result = longest(string1.as_str(), string2);
//
//     println!("The longest string is {}", result);
// }

// fn ten_twentyone() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";
//
//     let result = longest(string1.as_str(), string2);
//
//     println!("The longest string is {}", result);
//
//     fn longest(x: &str, y: &str) -> &str {
//         if x.len() > y.len {
//             x
//         } else {
//             y
//         }
//     }
// }

fn ten_twentytwo() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);

    println!("The longest string is {}", result);
}

fn ten_twentythree() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());

        println!("The longest string is {}", result);
    }
}

// fn ten_twentyfour() {
//     fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//         if x.len() > y.len() {
//             x
//         } else {
//             y
//         }
//     }
//
//     let string1 = String::from("long string is long");
//
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//
//     println!("The longest sting is {}", result);
// }

struct ImportantExcept<'a> {
    part: &'a str,
}

fn ten_twentyfive() {


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentance = novel.split('.').next().expect("Could no find a '.'");

    let i = ImportantExcept { part: first_sentance };

}

fn ten_twentysix() {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
}

fn main() {
    // ten_seventeen();
    // ten_eighteen();
    ten_nineteen();
    // ten_twenty();
    // ten_twentyone();
    ten_twentytwo();
    ten_twentythree();
    // ten_twentyfour();
    ten_twentyfive();
    ten_twentyfive();
}
