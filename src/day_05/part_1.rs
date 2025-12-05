use crate::input_util::read_lines;
use std::collections::VecDeque;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub fn solve() {
    let mut lines: VecDeque<String> = read_lines("src/day_05/input.txt").into();
    let mut ranges: Vec<RangeInclusive<i64>> = Vec::new();
    while lines[0].contains("-") {
        let string = lines.pop_front().unwrap();
        let mut parts = string.split("-");
        let range = i64::from_str(parts.next().unwrap()).unwrap()
            ..=i64::from_str(parts.next().unwrap()).unwrap();
        ranges.push(range);
    }
    lines.pop_front();
    let ans = lines
        .iter()
        .map(|s| i64::from_str(s).unwrap())
        .filter(|id| ranges.iter().any(|r| r.contains(&id)))
        .count();
    println!("{ans}");
}
