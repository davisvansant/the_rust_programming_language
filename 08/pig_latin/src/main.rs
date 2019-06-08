fn pig_latin(word: &mut String) -> String {
    let first_letter = word.chars().next().unwrap();
    if first_letter == 'a' ||
        first_letter == 'e' ||
        first_letter == 'i' ||
        first_letter == 'o' ||
        first_letter == 'u' ||
        first_letter == 'y' {
        word.push_str("-");
        word.push_str("hay");
        word.to_string()
    } else {
        let remove = word.remove(0);
        word.push_str("-");
        word.push(remove);
        word.push_str("ay");
        word.to_string()
    }
}

fn main() {
    // let mut word = String::from("fishin");
    let mut word = String::from("apple");
    println!("{:?}", pig_latin(&mut word));
}
