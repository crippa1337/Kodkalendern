use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut latest_3: Vec<u32> = vec![];

    for (locker_nr, line) in input.lines().enumerate() {
        latest_3.push(line.parse::<u32>().unwrap());

        if latest_3.len() == 3 {
            if latest_3[0] + latest_3[1] != latest_3[2] {
                let missing_amount = (latest_3[0] + latest_3[1]) - latest_3[2];
                println!("Locker [{locker_nr}] is missing [{missing_amount}]");
            }
            latest_3.clear();
        }
    }
}
