use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    for (index, line) in input.lines().enumerate() {
        let mut team: Vec<char> = vec![];
        let mut saved_team: Vec<char> = vec![];
        let mut pair_sums: Vec<u32> = vec![];

        // creates team
        for elf in line.chars() {
            team.push(elf);
            saved_team.push(elf)
        }

        // creates pair sums, outer is lower index
        for _ in 0..3 {
            pair_sums.push(get_outer_pair_sum(&team));
            pop_pair(&mut team);
        }

        if pair_sums[0] < pair_sums[1] && pair_sums[1] < pair_sums[2] {
            println!("Optimal team found at: {index} with team: {saved_team:?}")
        }
    }
}

fn get_outer_pair_sum(team: &Vec<char>) -> u32 {
    let length = team.len();
    let sum = team[0].to_digit(10).unwrap() + team[length - 1].to_digit(10).unwrap();
    return sum;
}

fn pop_pair(team: &mut Vec<char>) -> Vec<char> {
    let length = team.len();
    team.remove(length - 1);
    team.remove(0);
    return team.to_vec();
}
