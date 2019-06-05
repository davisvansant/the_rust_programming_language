fn eight_eleven() {
    let mut s = String::new();
    println!("{:?}", s);
}

fn eight_twelve() {
    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    println!("{:?} {:?}", data, s);
}

fn eight_thirteen() {
    let s = String::from("initial contents");

    println!("{:?}", s);
}

fn eight_fifteen() {
    let mut s = String::from("foo");
    s.push_str("bar");

    println!("{:?}", s);
}

fn eight_sixteen() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {:?}", s2);
}

fn eight_seventeen() {
    let mut s = String::from("lo");
    s.push('l');

    println!("{:?}", s);
}

fn eight_eighteen() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    println!("{:?}", s3);

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    let s = format!("{}-{}-{}", s4, s5, s6);
    println!("{:?}", s);
}

fn eight_nineteen() {
    // If I understand correctly, this wont work simply because Rust strings do not support indexing...
    // It seems its a "bad" idea to index strings because its unclear what the return value of the indexing
    // operation should be
    // let s1 = String::from("Hello");
    // let h = s1[0];
    for c in "hello hello".chars() {
        println!("{:?}", c);
    }
    for b in "learning stuffs".bytes() {
        println!("{:?}", b);
    }
}

fn eight_twenty() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
}

fn eight_twentyone() {
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);
}

fn eight_twentytwo() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);

    println!("{:?}", map);
}

fn eight_twentythree() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (k, v) in &scores {
        println!("{} : {}", k, v);
    }
}

fn eight_twentyfour() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}

fn eight_twentyfive() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn eight_twentysix(){
    use std::collections::HashMap;

    let text = "Hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn main() {
    eight_eleven();
    eight_twelve();
    eight_thirteen();
    eight_fifteen();
    eight_sixteen();
    eight_seventeen();
    eight_eighteen();
    eight_nineteen(); //strings are complex!
    eight_twenty();
    eight_twentyone();
    eight_twentytwo();
    eight_twentythree();
    eight_twentyfour();
    eight_twentyfive();
    eight_twentysix();
}
