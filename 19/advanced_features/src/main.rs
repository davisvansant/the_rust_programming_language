// raw pointers

fn nineteen_one() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}

fn nineteen_two() {
    let address = 0x012345usize;
    let r = address as *const i32;
}

fn nineteen_three() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is : {}", *r1);
        println!("r2 is : {}", *r2);
    }
}

//unsafe functions

fn nineteen_four() {
    let mut v = vec![1, 2, 3, 4, 5];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// fn nineteen_five() {
//     fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//         let len = slice.len();
//
//         assert!(mid <= len);
//
//         (&mut slice [..mid], &mut slice[mid..])
//     }
// }

fn nineteen_six() {
    use std::slice;

    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
        }
    }
}

fn nineteen_seven() {
    use std::slice;

    let address = 0x012345usize;
    let r = address as *mut i32;

    let sice = unsafe {
        slice::from_raw_parts_mut(r, 10000)
    };
}

// Foreign Function Interface

fn nineteen_eight() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// Global Variables

fn nineteen_nine() {
    static HELLO_WORLD: &str = "Hello, world!";

    println!("name is: {}", HELLO_WORLD);
}

fn nineteen_ten() {
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// Unsafe Trait

fn nineteen_eleven() {
    unsafe trait Foo {
        //
    }

    unsafe impl Foo for i32 {
        //
    }
}

// Lifetime subtyping

// fn nineteen_twelve() {
//     struct Context(&str);
//
//     struct Parser {
//         context: &Context,
//     }
//
//     impl Parser {
//         fn parse(&self) -> Result<(), &str> {
//             Err(&self.context.0[1..])
//         }
//     }
// }

fn nineteen_thirteen() {
    struct Context <'a>(&'a str);

    struct Parser<'a> {
        context: &'a Context<'a>,
    }

    impl <'a> Parser<'a> {
        fn parse(&self) -> Result<(), &str> {
            Err(&self.context.0[1..])
        }
    }
}

// fn nineteen_fourteen() {
//     struct Context <'a>(&'a str);
//
//     struct Parser<'a> {
//         context: &'a Context<'a>,
//     }
//
//     impl <'a> Parser<'a> {
//         fn parse(&self) -> Result<(), &str> {
//             Err(&self.context.0[1..])
//         }
//     }
//
//     fn parse_context(context: Context) -> Result<(), &str> {
//         Parser { context: &context }.parse()
//     }
// }

fn nineteen_fifteen() {
    struct Context<'s>(&'s str);

    struct Parser<'c, 's: 'c> {
        context: &'c Context<'s>,
    }

    impl<'c, 's> Parser<'c, 's> {
        fn parse(&self) -> Result<(), &'s str> {
            Err(&self.context.0[1..])
        }
    }

    fn parse_context(context: Context) -> Result<(), &str> {
        Parser { context: &context }.parse()
    }
}

// Lifetime bounds

fn nineteen_sixteen() {
    struct Ref<'a, T>(&'a T);
}

fn nineteen_seventeen() {
    struct Ref<'a, T: 'a>(&'a T);
}

fn nineteen_eighteen() {
    struct StaticRef<T: 'static>(&'static T);
}

// fn nineteen_nineteen() {
//     trait Red { }
//
//     struct Ball<'a> {
//         diameter: &'a i32,
//     }
//
//     impl<'a> Red for Ball<'a> { }
//
//     let num = 5;
//     let obj = Ball::new(Ball { diameter: &num }) as Box<Red>;
// }

// Associated Types

fn nineteen_twenty() {
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }
}

fn nineteen_twentyone() {
    pub trait Iterator<T> {
        fn enxt(&mut self) -> Option<T>;
    }
}

// Operator Overloading

fn nineteen_twentytwo() {
    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(Point { x: 1, y: 0 } + Point {x: 2, y: 3},
        Point {x: 3, y: 3});
}

fn nineteen_twentythree() {
    use std::ops::Add;

    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
}

// Fully Qualified Syntax

fn nineteen_twentyfour() {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking");
        }
    }

    impl Wizard for Human {
        fn fly (&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("waving arms furiously");
        }
    }
}

fn nineteen_twentyfive() {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking");
        }
    }

    impl Wizard for Human {
        fn fly (&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("waving arms furiously");
        }
    }

    let person = Human;
    person.fly();
}

fn nineteen_twentysix() {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking");
        }
    }

    impl Wizard for Human {
        fn fly (&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("waving arms furiously");
        }
    }

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

fn nineteen_twentyseven() {
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("A baby dog is called a {}", Dog::baby_name());
}

// fn nineteen_twentyeight() {
//     trait Animal {
//         fn baby_name() -> String;
//     }
//
//     struct Dog;
//
//     impl Dog {
//         fn baby_name() -> String {
//             String::from("Spot")
//         }
//     }
//
//     impl Animal for Dog {
//         fn baby_name() -> String {
//             String::from("puppy")
//         }
//     }
//
//     println!("A baby dog is called a {}", Animal::baby_name());
// }

fn nineteen_twentynine() {
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

// Supertrait

fn nineteen_thirty() {
    use std::fmt;

    struct Point {
        x: i32,
        y: i32
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}","*".repeat(len + 4));
        }
    }

    let asterisks = Point { x: 10, y: 30 };
    println!("{:?}", asterisks.outline_print());
}

// Newtype Pattern

fn nineteen_thirtyone() {
    use std::fmt;

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("ello"), String::from("world")]);
    println!("w = {}", w);
}

// fn nineteen_thirtytwo() {
//     let f: Box<Fn() + Send + 'static> = Box::new(|| println!("hi"));
//
//     fn takes_long_type(f: Box<Fn() + Send + 'static>) {
//         //
//     }
//
//     fn returns_long_type() -> Box<Fn() + Send + 'static> {
//         //
//     }
// }

// fn nineteen_thirtythree() {
//     type Thunk = Box<Fn() + Send + 'static>;
//
//     let f: Thunk = Box::new(|| println!("hi"));
//
//     fn takes_long_type(f: Thunk) {
//         //
//     }
//
//     fn returns_long_type() -> Thunk {
//         //
//     }
// }

// Empty Type

// fn nineteen_thirtyfour() {
//     let guess: u32 = match guess.trim().parse() {
//         Ok(num) => num,
//         Err(_) => continue,
//     };
// }

// Dynamically Sized Types

// fn generic <T: ?Sized(t: &T)> {
//
//}

// Function Pointers

fn nineteen_thirtyfive() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);

    println!("The answer is {}", answer);
}

fn main() {
    nineteen_one();
    nineteen_two();
    nineteen_three();
    // nineteen_four();
    nineteen_six();
    nineteen_seven();
    nineteen_eight();
    nineteen_nine();
    nineteen_ten();
    nineteen_eleven();
    // nineteen_twelve();
    nineteen_thirteen();
    // nineteen_fourteen();
    nineteen_fifteen();
    nineteen_sixteen();
    nineteen_seventeen();
    nineteen_eighteen();
    // nineteen_nineteen();
    nineteen_twenty();
    nineteen_twentyone();
    nineteen_twentytwo();
    nineteen_twentythree();
    nineteen_twentyfour();
    nineteen_twentyfive();
    nineteen_twentysix();
    nineteen_twentyseven();
    // nineteen_twentyeight();
    nineteen_twentynine();
    nineteen_thirty();
    nineteen_thirtyone();
    // nineteen_thirtytwo();
    // nineteen_thirtythree();
    // nineteen_thirtyfour();
    nineteen_thirtyfive();
}
