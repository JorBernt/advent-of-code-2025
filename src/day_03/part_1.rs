use crate::input_util::read_lines;

pub fn solve() {
    let banks: Vec<Vec<u32>> = read_lines("src/day_03/input.txt")
        .iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut sum = 0;
    for bank in banks {
        let max = bank[0..bank.len() -1].iter().max().unwrap();
        let max_index = bank.iter().position(|n| n == max).unwrap();
        let sub = &bank[max_index+1..bank.len()];
        let second_max = sub.iter().max().unwrap();
        sum += max * 10 + second_max;
    }
    println!("{sum}");
}
