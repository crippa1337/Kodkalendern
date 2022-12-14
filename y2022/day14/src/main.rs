use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut all_circums: HashMap<u32, u32> = HashMap::new();

    for line in input.lines() {
    for line in input.lines() {
        let circum: u32 = line
            .split(", ")
            .map(|x| x.parse::<u32>().unwrap())
            .fold(0, |accum, num| (num * 2) + accum);

        *all_circums.entry(circum).or_default() += 1;
    }

    let no_pair = all_circums.iter().fold((&0u32, &0u32), |no_pair, pair| {
        if no_pair.1 % 2 != 0 {
            return no_pair;
    let no_pair = all_circums.iter().fold((&0u32, &0u32), |no_pair, pair| {
        if no_pair.1 % 2 != 0 {
            return no_pair;
        }
        return pair;
        return pair;
    });

    // Elf key
    println!("The elf with no pair: {}", no_pair.0)
    // Elf key
    println!("The elf with no pair: {}", no_pair.0)
    // 152
}
