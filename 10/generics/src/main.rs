fn ten_one(){
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is : {:?}", largest);
}

fn ten_two() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is : {:?}", largest);

    let number_list = vec![102, 34, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is : {:?}", largest);

}

fn ten_three() {
    fn largest(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is : {:?}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is : {:?}", result);
}

fn ten_four() {
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is : {:?}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is : {:?}", result);
}

// fn ten_five() {
//     fn largest<T>(list: &[T]) -> T {
//         let mut largest = list[0];
//
//         for &item in list.iter() {
//             if item > largest {
//                 largest = item;
//             }
//         }
//         largest
//     }
//
//     let number_list = vec![34, 50, 25 ,100, 65];
//     let result = largest(&number_list);
//     println!("The largest number is {:?}", result);
//
//     let char_list = vec!['y', 'm', 'a', 'q'];
//     let result = largest(&char_list);
//     println!("The larget char is {:?}", result);
// }

fn ten_six() {
    struct Point<T> {
        x: T,
        y: T
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}

// fn ten_seven() {
//     struct Point<T> {
//         x: T,
//         y: T,
//     }
//
//     let wont_work = Point { x:5, y: 4.0 };
// }

fn ten_eight() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}

fn ten_nine() {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

// fn ten_ten() {
//     impl Point<f32> {
//         fn distance_from_origin(&self) -> f32 {
//             (self.x.powi(2) + self.y.power(2)).sqrt()
//         }
//     }
// }

fn ten_eleven() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W> (self, other: Point<V, W>) -> Point<T,W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {:?}, p3.y = {:?}", p3.x, p3.y);
}

fn main() {
    ten_one();
    ten_two();
    ten_three();
    ten_four();
    // ten_five();
    ten_six();
    // ten_seven();
    ten_eight();
    ten_nine();
    // ten_ten();
    ten_eleven();

}
