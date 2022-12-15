use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut answer: u32 = 0;

    for line in input.lines() {
        let mut evens: u8 = 0;
        let mut odds: u8 = 0;
        for char in line.chars().map(|x| x.to_digit(10).unwrap()) {
            match char % 2 {
                0 => evens += 1,
                1 => odds += 1,
                _ => unreachable!(),
            }
        }

        if odds != evens {
            answer += 1;
        }
    }

    println!("Answer: {answer}");
    // 19
}
