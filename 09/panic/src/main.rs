fn nine_one() {
    panic!("at the disco");
}

fn nine_two() {
    let v = vec![1, 2, 3];

    v[99];
}

// fn nine_three() {
//     use std::fs::File;
//
//     let f: u32 = File::open("hello.txt");
// }

// fn nine_four() {
//     use std::fs::File;
//
//     let f = File::open("hello.txt");
//
//     let f = match f {
//         Ok(file) => file,
//         Err(error) => {
//             panic!("There was an error opening the file: {:?}", error)
//         },
//     };
// }

fn nine_five() {
    use std::fs::File;
    use std::io::ErrorKind;

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
}

fn nine_six() {
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
}

fn nine_seven() {
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
}

fn nine_eight() {
    use std::io;
    use std::io::Read;
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }
}

fn nine_nine() {
    pub struct Guess{
        value: u32,
    }

    impl Guess {
        pub fn new(value: u32) -> Guess{
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}", value);
            }

            Guess {
                value
            }
        }

        pub fn value(&self) -> u32 {
            self.value
        }
    }
}

fn main() {
    // nine_one();
    // nine_two();
    // nine_three();
    // nine_four();
    // nine_five();
    // nine_six();
    nine_seven();
    nine_eight();
    nine_nine();
}
