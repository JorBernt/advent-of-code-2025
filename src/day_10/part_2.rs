use crate::input_util::read_lines;
use std::collections::HashMap;
use std::ffi::c_ushort;
use std::os::unix::raw::uid_t;
use std::pin::Pin;

#[derive(Debug)]
struct Light {
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u64>,
}
pub fn solve() {
    let lines = read_lines("src/day_10/input.txt")
        .iter()
        .map(|l| {
            let mut buttons = l
                [l.find(|c| c == ']').unwrap() + 1..l.find(|c| c == '{').unwrap() - 1]
                .split_whitespace()
                .map(|s| {
                    (&s[1..s.len() - 1])
                        .split(",")
                        .map(|d| d.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();
            buttons.sort_by(|a, b| b.len().cmp(&a.len()));
            let joltage = l[l.find(|c| c == '{').unwrap() + 1..l.find(|c| c == '}').unwrap()]
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u64>>();
            Light { buttons, joltage }
        })
        .collect::<Vec<Light>>();
    let ans = lines
        .iter()
        .enumerate()
        .map(|(i, l)| {
            let mut k = 0;
            for j in 0..l.buttons.len() {
                println!("{i}/{}", lines.len());
                let memo = &mut HashMap::new();
                k = calc(l, j, &mut vec![0; l.joltage.len()], 0, memo);
                println!("{k}");
                if k != 0 {
                    break
                }
            }
            k
        })
        .collect::<Vec<u64>>();
    println!("{}", ans.iter().sum::<u64>());
}

fn calc(
    light: &Light,
    index: usize,
    current: &mut Vec<u64>,
    count: u64,
    memo: &mut HashMap<Vec<u64>, u64>,
) -> u64 {
    if current == &light.joltage {
        return count;
    }
    if memo.contains_key(current) {
        return 0
    }
    memo.insert(current.clone(), 0);
    for i in 0..light.buttons.len() {
        let b = &light.buttons[(i + index) % light.buttons.len()];
        if !b.iter().all(|n| current[*n] < light.joltage[*n]) {
            continue;
        }
        let max = b
            .iter()
            .map(|n| current[*n].abs_diff(light.joltage[*n]))
            .min()
            .unwrap();
        for n in b {
            current[*n] += max;
        }
        for _ in 0..max{
            let m = calc(light,index, current, count + max, memo);
            if m != 0 {
                return m
            }
            for n in b {
                current[*n] -= 1;
            }
        }

    }
     println!("{:?}", current);
    0
}
