fn eight_one() -> Vec<i32> {
    let v: Vec<i32> = Vec::new();
    v
}

fn eight_two() -> Vec<i32> {
    let v = vec![1, 2, 3];
    v
}

fn eight_three() -> Vec<i32> {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v
}

fn eight_four<'a>() -> i32 {
    let v = vec![1, 2, 3, 4, 5];
    v[4]
}

fn eight_eight() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{:?}", i);
    }
}

fn eight_nine() {
    let mut v = vec![100, 39, 20920, 902, 3590232];
    for i in &mut v {
        *i += 50;
        println!("{:?}", i);
    }
}

fn eight_ten() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("cool stuff!")),
        SpreadsheetCell::Float(10.12),
    ];

    for enums in row {
        println!("{:?}", enums);
    }

}

fn main() {
    println!("{:?}", eight_one());
    println!("{:?}", eight_two());
    println!("{:?}", eight_three());
    println!("{:?}", eight_four());
    eight_eight();
    eight_nine();
    eight_ten();
}
