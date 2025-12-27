use crate::input_util::read_lines;
use std::collections::HashMap;

#[derive(Debug)]
struct Light {
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u64>,
}
pub fn solve() {
    let lines = read_lines("src/day_10/input.txt")
        .iter()
        .map(|l| {
            let buttons = l[l.find(|c| c == ']').unwrap() + 1..l.find(|c| c == '{').unwrap() - 1]
                .split_whitespace()
                .map(|s| {
                    (&s[1..s.len() - 1])
                        .split(",")
                        .map(|d| d.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();
            let joltage = l[l.find(|c| c == '{').unwrap() + 1..l.find(|c| c == '}').unwrap()]
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u64>>();
            Light { buttons, joltage }
        })
        .collect::<Vec<Light>>();
    let mut ans = 0;
    for mut l in lines {
        let a = calc(&mut l.joltage, &l.buttons, &mut HashMap::new());
        println!("{a}");
        ans += a;
    }
    print!("{ans}");
    fn calc(
        joltage: &mut Vec<u64>,
        buttons: &Vec<Vec<usize>>,
        memo: &mut HashMap<i64, Vec<Vec<Vec<usize>>>>,
    ) -> i64 {
        if joltage.iter().all(|n| n == &0) {
            return 0;
        }
        let target = convert_to_target(joltage);

        if !memo.contains_key(&target) {
            combinations(
                target,
                0,
                0,
                joltage.len(),
                buttons,
                &mut Vec::new(),
                memo,
                true,
            );
        }
        let mut m = 1_000_000;
        if target == 0 {
            print!("");
        }
        let x = &memo[&target].clone();
        if target == 0 {
            let mut jolt = joltage.clone();
            for i in 0..jolt.len() {
                jolt[i] /= 2;
            }
            m = m.min((calc(&mut jolt, buttons, memo) * 2))
        }
        'outer: for c in x {
            let mut jolt = joltage.clone();
            for k in c {
                for n in k {
                    if jolt[*n] == 0 {
                        continue 'outer;
                    } else {
                        jolt[*n] -= 1;
                    }
                }
            }

            for i in 0..jolt.len() {
                jolt[i] /= 2;
            }
            if m < 70 {
                print!("");
            }
            m = m.min((calc(&mut jolt, buttons, memo) * 2) + c.len() as i64)
        }
        m
    }
}
fn convert_to_target(joltage: &Vec<u64>) -> i64 {
    let mut l = joltage.len();
    let mut target = 0;
    for n in joltage {
        l -= 1;
        target += (n & 1) << l;
    }
    target as i64
}

fn combinations(
    target: i64,
    current: i64,
    index: usize,
    length: usize,
    buttons: &Vec<Vec<usize>>,
    combs: &mut Vec<usize>,
    selected: &mut HashMap<i64, Vec<Vec<Vec<usize>>>>,
    first: bool,
) {
    if !first && current == target {
        if combs.len() > 0 {
            if !selected.contains_key(&target) {
                selected.insert(target, Vec::new());
            }
            let mut a = Vec::new();
            let x = selected.get_mut(&target).unwrap();
            for n in combs {
                a.push(buttons[*n].clone());
            }
            x.push(a);
        }
        return;
    }
    if index == buttons.len() {
        return;
    }
    let b = &buttons[index];
    let n = b.iter().fold(0, |acc, n| acc + (1 << ((length - 1) - n)));
    combs.push(index);
    combinations(
        target,
        current ^ n,
        index + 1,
        length,
        buttons,
        combs,
        selected,
        false,
    );
    combs.pop();
    combinations(
        target,
        current,
        index + 1,
        length,
        buttons,
        combs,
        selected,
        false,
    );
}
