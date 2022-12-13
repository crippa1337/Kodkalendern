use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut answer: u32 = 0;
    for line in input.lines() {
        let vec: Vec<&str> = line.split(", ").collect();
        let num_vec: Vec<u32> = vec
            .into_iter()
            .map(|string| string.parse::<u32>().unwrap())
            .collect();

        if !num_vec.contains(&7) {
            let volume: u32 = num_vec
                .iter()
                // Could use .product() here
                .fold(1, |product, measurement| measurement * product);
            answer += volume;
        }
    }

    println!("{answer}");
    // 59193
}
