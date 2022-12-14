use std::fs;

fn stov(string: &str) -> Vec<&str> {
    let splitted: Vec<&str> = string.split(", ").collect();
    return splitted;
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut iter = input.lines();
    let vowels = stov(iter.next().unwrap());
    let consonants = stov(iter.next().unwrap());
    let poem = iter.next().unwrap();

    println!("Vowels: {vowels:?}\nConsonants: {consonants:?}");
    let mut v_amount: i32 = 0;
    let mut c_amount: i32 = 0;
    for char in poem.chars() {
        if vowels.contains(&char.to_string().as_str()) {
            v_amount += 1;
        } else if consonants.contains(&char.to_string().as_str()) {
            c_amount += 1;
        } else {
        }
    }

    println!("The difference is: {}", (v_amount - c_amount).abs())
    // 1, answer yes
}
