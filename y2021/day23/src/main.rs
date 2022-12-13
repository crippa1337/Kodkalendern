use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    for line in input.lines() {
        let mut line_chars: Vec<char> = vec![];
        let mut sorted_chars: Vec<char> = vec![];

        for chars in line.chars() {
            line_chars.push(chars);
            sorted_chars.push(chars);
        }
        sorted_chars.sort();

        let length = line_chars.len();
        if line_chars[length - 1].is_alphabetic() && line_chars[length - 2].is_alphabetic() {
            println!("Answer: {line}");
            // Doesn't return anything
        }

        for (char1, char2) in line_chars.iter().zip(sorted_chars.iter()) {
            if char1 != char2 {
                println!("Answer: {line}")
                // 124379ö
            }
        }
    }
}
