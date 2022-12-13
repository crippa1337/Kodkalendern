use std::{fs, vec};

fn main() {
    let input = fs::read_to_string("input").unwrap();

    for line in input.lines() {
        let mut line_chars: Vec<char> = vec![];

        for chars in line.chars() {
            line_chars.push(chars)
        }
    }
}

// Input is incorrectly parsed
