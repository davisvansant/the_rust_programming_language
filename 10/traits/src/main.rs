fn ten_twelve() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }
}

fn ten_thirteen() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet : {:?}", tweet.summarize());
}

fn ten_fourteen() {
    pub trait Summary {
        fn summarize_author(&self) -> String;

        // fn summarize(&self) -> String {
        //     String::from("(Read more...)")
        // }
        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        // fn summarize(&self) -> String {
        //     format!("{}, by {} ({})", self.headline, self.author, self.location)
        // }
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    // impl Summary for Tweet {
    //     fn summarize(&self) -> String {
    //         format!("{}: {}", self.username, self.content)
    //     }
    // }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet : {:?}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Blues win the Stanley Cup Championship!"),
        location: String::from("St Louis, MO, USA"),
        author: String::from("Louie"),
        content: String::from("The St Louis Blues have won the Stanley Cup for the first time in franchise history!"),
    };

    println!("New article available! {}", article.summarize());
}

fn ten_fifteen() {
    // fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    //     let mut largest = list[0];
    //
    //     for &item in list.iter() {
    //         if item > largest {
    //             largest = item
    //         }
    //     }
    //     largest
    // }
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list.iter() {
            if item > largest {
                largest = item
            }
        }
        &largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}",  result);
}

fn ten_sixteen() {
    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {
                x,
                y,
            }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}

fn main() {
    ten_twelve();
    ten_thirteen();
    ten_fourteen();
    ten_fifteen();
    ten_sixteen();
}
