use std::fs;

fn minutes_to_seconds(minutes: u32) -> u32 {
    return minutes * 60;
}

fn amount_of_frames(length: u32) -> u32 {
    return length * 24;
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let video_length = minutes_to_seconds(83);
    let frames = amount_of_frames(video_length);

    for (index, line) in input.lines().enumerate() {
        if frames == line.parse::<u32>().unwrap() {
            println!("Film number: {}", index + 1)
            // 765
        }
    }
}
