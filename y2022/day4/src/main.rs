use std::fs;

fn celcius_to_fahrenheit(fahrenheit: f32) -> f32 {
    let celcius = (fahrenheit - 32.0) * (5.0 / 9.0);
    return celcius;
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let mut answer: u16 = 0;
    for line in input.lines() {
        let celcius = celcius_to_fahrenheit(line.parse().unwrap());
        if celcius < 5.0 && celcius > 0.0 {
            answer += 1;
        }
    }

    println!("{answer}");
    // 32
}
