use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    for (elf, line) in input.lines().enumerate() {
        let vec: Vec<&str> = line.split(",").collect();
        let length: u32 = vec[0].trim().parse().unwrap();
        let height: u32 = vec[1].trim().parse().unwrap();
        let input_area: u32 = vec[2].trim().parse().unwrap();
        let real_area: u32 = length * height;

        if input_area != real_area {
            println!("{elf}");
            // 77, 123, 911
        }
    }
}
