use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let before_santa: Vec<char> = input.chars().collect();
    let mut after_santa: Vec<char> = Vec::new();

    for char in before_santa.iter() {
        // Reverse talking elves direction
        if *char == '<' {
            after_santa.push('>');
            continue;
        } else if *char == '>' {
            after_santa.push('<');
            continue;
        }
        // Push quiet elves
        after_santa.push(*char);
    }

    let answer = check_talkers(after_santa);
    println!("{answer}")
    // 510
}

fn check_talkers(queue: Vec<char>) -> u32 {
    let mut talkers = 0;
    for (index, _) in queue.iter().enumerate() {
        if queue[index] == '<' && queue[index + 1] == '>' {
            // Add a pair of talkers
            talkers += 2
        }
    }

    return talkers;
}
