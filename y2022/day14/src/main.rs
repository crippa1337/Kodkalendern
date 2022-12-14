use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut all_circums: HashMap<u32, u32> = HashMap::new();

    for (index, line) in input.lines().enumerate() {
        let circum: u32 = line
            .split(", ")
            .map(|x| x.parse::<u32>().unwrap())
            .fold(0, |accum, num| (num * 2) + accum);

        *all_circums.entry(circum).or_default() += 1;
    }

    let max = all_products.iter().fold((&0u32, &0u32), |curr_max, prod| {
        if prod.1 > curr_max.1 {
            return prod;
        }
        return curr_max;
    });

    println!("{all_circums:?} and {}", all_circums.len())
}
