use crate::input_util::read_lines;
use std::cmp::Ordering;
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
    ranges.sort_by(|a, b| b.start().cmp(a.start()));
    let mut reduced_ranges: Vec<RangeInclusive<i64>> = Vec::new();
    while !ranges.is_empty() {
        let range = ranges.pop().unwrap();
        if let Some(peek) = reduced_ranges.pop_if(|r| r.contains(range.start())) {
            if peek.contains(range.end()) {
                reduced_ranges.push(peek);
                continue;
            }
            reduced_ranges.push(*peek.start()..=*range.end());
        } else {
            reduced_ranges.push(range);
        }
    }
    let mut ans = 0;
    reduced_ranges
        .iter()
        .for_each(|r| ans += r.end() - r.start() + 1);
    print!("{ans}");
}
