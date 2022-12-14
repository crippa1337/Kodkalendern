use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut all_products: HashMap<u32, u32> = HashMap::new();
    for line in input.lines() {
        let product = line
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u32)
            .fold(1, |prod, factor| factor * prod);

        *all_products.entry(product).or_default() += 1;
    }

    let max = all_products.iter().fold((&0u32, &0u32), |curr_max, prod| {
        if prod.1 > curr_max.1 {
            return prod;
        }
        return curr_max;
    });

    println!("{max:?}")
}
