fn main() {
    let mut length = 0;
    let mut throw = 0;
    let mut throw_adder = 0;
    while throw < 100 {
        length += throw;
        throw_adder += 1;
        throw += throw_adder;
    }
    println!("{length}");
    // 455
}
