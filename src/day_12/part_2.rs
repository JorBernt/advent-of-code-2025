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
    let ans = traverse("svr", &devices, &mut HashMap::new(), false, false);
    println!("{ans}");
}

fn traverse(current: &str, devices: &HashMap<&str, Vec<&str>>, dp: &mut HashMap<(String, bool, bool), u64>, mut fft: bool, mut dac: bool) -> u64 {
    if current == "out" {
        if fft && dac {
            return 1
        }
        return 0;
    }
    let x = (current.into(), fft, dac);
    if dp.contains_key(&x) {
        return dp[&x];
    }
    if current == "fft" {
        fft = true;
    }
    if current == "dac" {
        dac = true;
    }
    let m = devices[current].iter().map(|d|traverse(d, devices, dp, fft, dac)).sum();
    dp.insert((current.to_string(), fft, dac), m);
    m
}