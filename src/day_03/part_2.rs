use crate::input_util::read_lines;

pub fn solve() {
    let banks: Vec<Vec<u32>> = read_lines("src/day_03/input.txt")
        .iter()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut sum : u64 = 0;
    for bank in banks {
        let mut index = 0;
        let mut window_size = bank.len() - 11;
        let mut exp : u64 = 100000000000;
        let mut part_sum = 0;
        while window_size > 0 && exp > 0 {
            let bank_window = &bank[index..(index + window_size).min(bank.len())];
            let max = bank_window.iter().max().unwrap();
            let max_index = bank_window.iter().position(|n|n == max).unwrap();
            window_size -= max_index;
            index += max_index + 1;
            part_sum += *max as u64 * exp;
            exp /= 10;
        }
        //println!("{part_sum}");
        sum += part_sum;
    }
    println!("{sum}");
}
