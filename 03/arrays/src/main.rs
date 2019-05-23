fn main() {
    let a = [1,2,3,4,5];

    for num in a.iter() {
        println!("num is {}", num);
    }

    println!("first is {}", a[0]);
    println!("second is {}", a[1]);
    println!("third is {}", a[2]);
    println!("fourth is {}", a[3]);
    println!("fifth is {}", a[4]);
    //println!("broke is {}", a[10]);
}
