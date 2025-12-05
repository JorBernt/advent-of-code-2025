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
            for i in 1..=str.len() / 2 {
                if str.len() % i != 0 {
                    continue;
                }
                let parts = str.as_bytes()
                    .chunks(i)
                    .map(str::from_utf8)
                    .collect::<Result<Vec<&str>, _>>()
                    .unwrap();
                let m = *parts.first().unwrap();
                if parts.iter().all(|s|*s == m) {
                    sum += n;
                    break;
                }
            }
        }
    }
    println!("{sum}");
}
