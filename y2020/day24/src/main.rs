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
}

fn check_talkers(queue: Vec<char>) -> u32 {
    let mut answer: u32 = 0;
    for (index, _) in queue.iter().enumerate() {
        let left: char;
        let middle: char;
        let right: char;

        // Left
        match queue.get(index - 1) {
            Some(_) => left = *queue.get(index - 1).unwrap(),
            None => left = ' ',
        }

        // Middle
        middle = *queue.get(index).unwrap();

        // Right
        match queue.get(index + 1) {
            Some(_) => right = *queue.get(index + 1).unwrap(),
            None => right = ' ',
        }

        if left == '<' && middle == '>' {
            answer += 1;
        }

        if middle == '<' && right == '>' {
            answer += 1;
        }
    }

    return answer;
}
