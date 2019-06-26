// fn fifteen_one() {
//     let b = Box::new(5);
//     println!("b = {}", b);
// }

// fn fifteen_two() {
//     enum List {
//         Cons(i32, List),
//         Nil
//     }
// }

// fn fifteen_three() {
//     enum List {
//             Cons(i32, List),
//             Nil
//         }
//
//     use List::{Cons, Nil};
//
//     let list = Cons(1, Cons(2, Cons(3,Nil)));
// }

// fn fifteen_five() {
//     enum List {
//         Cons(i32, Box<List>),
//         Nil,
//     }
//
//     use List::{Cons, Nil};
//
//     let list = Cons(1,
//         Box::new(Cons(2,
//             Box::new(Cons(3,
//                 Box::new(Nil))))));
// }
//
// fn fifteen_six() {
//     let x = 5;
//     let y = &x;
//
//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }
//
// fn fifteen_seven() {
//     let x = 5;
//     let y = Box::new(x);
//
//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }

// fn fifteen_eight() {
//     struct MyBox<T>(T);
//
//     impl<T> MyBox<T> {
//         fn new(x: T) -> MyBox<T> {
//             MyBox(x)
//         }
//     }
// }

// fn fifteen_nine() {
//     struct MyBox<T>(T);
//
//     impl<T> MyBox<T> {
//         fn new(x: T) -> MyBox<T> {
//             MyBox(x)
//         }
//     }
//
//     let x = 5;
//     let y = MyBox::new(x);
//
//     assert_eq!(5, x);
//     assert_eq!(5, *y)
// }

fn fifteen_ten() {
    use std::ops::Deref;

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }


    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y)
}

fn fifteen_eleven() {
    fn hello(name: &str) {
        println!("Hello {}!", name);
    }
}

fn fifteen_twelve() {
    use std::ops::Deref;

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    fn hello(name: &str) {
        println!("Hello {}!", name);
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn fifteen_fourteen() {
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data {}",  self.data);
        }
    }

    let c = CustomSmartPointer { data: String::from("my stuff")};
    let d = CustomSmartPointer { data: String::from("other stuff")};

    println!("CustomSmartPointers created.");
}

fn fifteen_sixteen() {
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data {}",  self.data);
        }
    }

    let c = CustomSmartPointer { data: String::from("some data")};
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
}

// fn fifteen_seventeen() {
//     enum List {
//         Cons(i32, Box<List>),
//         Nil,
//     }
//
//     use List::{Cons,Nil};
//
//     let a = Cons(5,
//         Box::new(Cons(10,
//             Box::new(Nil))));
//     let b = Cons(3, Box::new(a));
//     let c = Cons(4, Box::new(a));
// }

fn fifteen_eighteen() {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    use std::rc::Rc;

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(4, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

fn fifteen_nineteen() {
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    use std::rc::Rc;

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
}

fn fifteen_twentyfour() {
    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    use std::rc::Rc;
    use std::cell::RefCell;

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));

    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

fn fifteen_twentyfive() {
    use std::rc::Rc;
    use std::cell::RefCell;
    use List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match *self {
                Cons(_, ref item) => Some(item),
                Nil => None,
            }
        }
    }
}

fn fifteen_twentysix() {
    use std::rc::Rc;
    use std::cell::RefCell;
    use List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }

    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match *self {
                Cons(_, ref item) => Some(item),
                Nil => None,
            }
        }
    }

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    println!("a next item = {:?}", a.tail());
}

fn main() {
    // fifteen_one();
    // fifteen_two();
    // fifteen_three();
    // fifteen_five();
    // fifteen_six();
    // fifteen_seven();
    // fifteen_eight();
    // fifteen_nine();
    // fifteen_ten();
    // fifteen_eleven();
    // fifteen_twelve();
    // fifteen_fourteen();
    // fifteen_sixteen();
    // fifteen_seventeen();
    // fifteen_eighteen();
    // fifteen_nineteen();
    // fifteen_twentyfour();
    fifteen_twentysix();
}
