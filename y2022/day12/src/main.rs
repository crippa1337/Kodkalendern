fn main() {
    let mut elf_to_reindeer: Vec<u32> = vec![];
    for elf in 2..=400u32 {
        let elf_square = (elf * elf).to_string();
        match elf_square.split_once(&elf.to_string()) {
            Some(x) => {
                if x.1 == "" {
                    elf_to_reindeer.push(elf)
                }
            }
            _ => continue,
        }
    }

    println!("{:?}", elf_to_reindeer)
    // [5, 6, 25, 76, 376]
}
