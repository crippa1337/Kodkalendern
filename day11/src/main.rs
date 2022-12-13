use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut answer: u32 = 0;

    for line in input.lines() {
        let vec: Vec<&str> = line.split(", ").collect();
        for (char1, char2) in vec[0].chars().zip(vec[1].chars()) {
            println!("{:?}", vec);
            println!("char 1: {char1}\nchar 2: {char2}");
            if char1 != char2 {
                answer += 1;
            }
        }
    }

    println!("{answer}");
    // 856 (Input is incorrect)
}
