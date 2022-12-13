use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    for line in input.lines() {
        let line_numbers: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        let mut pair: Vec<u32> = vec![];

        for number in line_numbers.iter() {
            pair.push(*number);

            if pair.len() == 2 {
                let modulo = pair[0] % 2 + pair[1] % 2;
                if modulo == 1 {
                    println!("Answer: {line}");
                    // 601891
                }
                pair.clear();
            }
        }
    }
}
