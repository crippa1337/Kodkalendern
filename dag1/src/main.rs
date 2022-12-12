fn main() {
    let mut answer = 0;
    let mut sum: u32 = 0;
    let mut difference: u32 = 0;

    for i in 67..=95u32 {
        sum += i;
    }

    for i in 48..=63u32 {
        difference += i;
    }

    answer += sum - difference;
    println!("Answer: {answer}");
    // 1461
}
