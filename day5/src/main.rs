use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    for line in input.lines() {
        let vec: Vec<&str> = line.split(",").collect();
        let previous: u32 = vec[0].parse().unwrap();
        let powered_previous: u32 = previous * u32::pow(2, 5);
        let current: u32 = vec[1].trim().parse().unwrap();

        if current > powered_previous {
            println!(
                "Previous: {} becomes -> {}\nCurrent: {}",
                previous,
                previous * u32::pow(2, 5),
                current
            );
            // Yes, at (189414, 9456178)
        }
    }
}
