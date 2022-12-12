use std::fs;

fn years_to_months(years: u32) -> u32 {
    return years * 16;
}

fn months_to_days(months: u32) -> u32 {
    return months * 26;
}
fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut answer: u8 = 0;
    let minimum_age: u32 = 10_000;

    for line in input.lines() {
        let vec: Vec<&str> = line.split(",").collect();
        let years: u32 = vec[0].trim().parse().unwrap();
        let mut months: u32 = vec[1].trim().parse().unwrap();
        let mut days: u32 = vec[2].trim().parse().unwrap();
        months += years_to_months(years);
        days += months_to_days(months);

        if days >= minimum_age {
            answer += 1;
        }
    }

    println!("{answer}");
    // 32
}
