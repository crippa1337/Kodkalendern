use std::fs;

fn main() {
    let input = fs::read_to_string("dag_11_input.txt").unwrap();
    let mut answer: u32 = 0;

    for line in input.lines() {
        let vec: Vec<&str> = line.split(", ").collect();
        for (char1, char2) in vec[0].chars().zip(vec[1].chars()) {
            if char1 != char2 {
                answer += 1;
            }
        }
    }

    println!("{answer}");
    // 648
}
