use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut sacks: u32 = 0;
    for line in input.lines() {
        let gifts: u32 = line.parse().unwrap();
        // If it's divisible by 8, add the amount of even sacks that are needed
        if gifts % 8 == 0 {
            sacks += gifts / 8;
        // Otherwise, floor the amount of gifts and add one, if it's 6
        // it adds 1, if it's 12, it adds 1+1
        } else {
            sacks += gifts / 8 + 1;
        }
    }

    println!("{sacks}");
    // 3023
}
