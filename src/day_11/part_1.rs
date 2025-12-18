use std::collections::HashMap;
use crate::input_util::read_lines;

pub fn solve() {
    let mut devices = HashMap::new();
    let input = read_lines("src/day_11/input.txt");
    input
        .iter().for_each(|l| {
        let (key, values) = l.split_once(": ").unwrap();
        devices.insert(key, values.split_whitespace().collect::<Vec<&str>>());
    });
    let ans = traverse("you", &devices);
    println!("{ans}");
}

fn traverse(current: &str, devices: &HashMap<&str, Vec<&str>>) -> u32 {
    if current == "out" {
        return 1;
    }
    devices[current].iter().map(|d|traverse(d, devices)).sum()
}