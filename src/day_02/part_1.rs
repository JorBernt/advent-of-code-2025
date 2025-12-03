use crate::input_util::read_lines;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub fn solve() {
    let ranges: Vec<RangeInclusive<u64>> = read_lines("src/day_02/input.txt")
        .first()
        .unwrap()
        .split(",")
        .map(|l| {
            let r: Vec<u64> = l.split("-").map(|n| u64::from_str(n).unwrap()).collect();
            r[0]..=r[1]
        })
        .collect();
    let mut sum = 0;
    for range in ranges {
        for n in range {
            let str = n.to_string();
            if str[0..str.len()/2] == str[str.len()/2..str.len()] {
                sum += n;
            }
        }
    }
    println!("{sum}");
}
