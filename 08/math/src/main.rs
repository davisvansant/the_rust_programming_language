fn mean(n: &Vec<i32>) -> f32 {
    let sum: i32 = n.iter().sum();
    let len = n.iter().len();
    let mean = sum as f32 / len as f32;
    mean
}

fn median(n: &mut Vec<i32>) -> f32{
    n.sort();
    let middle = n.len() / 2;
    if n.len() % 2 == 0 {
        mean(&vec![n[middle -1], n[middle]]) as f32
    }
    else {
        n[middle] as f32
    }
}

fn mode(n: &Vec<i32>) {
    use std::collections::HashMap;
    let mut map:HashMap<i32, i32> = HashMap::new();
    for i in n {
        let counter = map.entry(*i).or_insert(0);
        *counter += 1;
    }
    let max_value = map.values().cloned().max().unwrap_or(0);
    println!("The mode for this dataset");
    for (k, v) in map {
        if v >= max_value && v > 1 {
            println!("{}", k);
        }
    }
}

fn main() {
    let mut numbers = vec![10, 8, 689, 32, 2309, 2309, 4209, 4209, 50438, 23, 7340, 23];
    println!("{:?}", mean(&numbers));
    println!("{:?}", median(&mut numbers));
    mode(&numbers);
}
