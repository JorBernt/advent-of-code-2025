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
        let mut mask = HashMap::new();
        combinations(
            0,
            0,
            l.joltage.len(),
            &l.buttons,
            &mut Vec::new(),
            &mut mask,
        );
        let a = calc(&mut l.joltage, &l.buttons, &mut mask, &mut HashMap::new());
        println!("{a}");
        ans += a;
    }
    print!("{ans}");
    fn calc(
        joltage: &mut Vec<u64>,
        buttons: &Vec<Vec<usize>>,
        masks: &mut HashMap<i64, Vec<(usize, Vec<usize>)>>,
        memo: &mut HashMap<Vec<u64>, i64>,
    ) -> i64 {
        if joltage.iter().all(|n| n == &0) {
            return 0;
        }
        if memo.contains_key(joltage) {
            return memo[joltage];
        }
        let target = convert_to_target(joltage);
        let mut m = 1_000_000;
        if !masks.contains_key(&target) {
            return m;
        }
        let x = &masks[&target].clone();

        'outer: for (presses, butt) in x {
            let mut jolt = joltage.clone();
            for n in butt {
                if (jolt[*n] as i64) - 1 < 0 {
                    continue 'outer;
                } else {
                    jolt[*n] -= 1;
                }
            }

            for i in 0..jolt.len() {
                jolt[i] /= 2;
            }
            m = m.min((calc(&mut jolt, buttons, masks, memo) * 2) + *presses as i64)
        }
        memo.insert(joltage.clone(), m);
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
    current: i64,
    index: usize,
    length: usize,
    buttons: &Vec<Vec<usize>>,
    combs: &mut Vec<usize>,
    mask: &mut HashMap<i64, Vec<(usize, Vec<usize>)>>,
) {
    if index == buttons.len() {
        if !mask.contains_key(&current) {
            mask.insert(current, Vec::new());
        }
        mask.get_mut(&current).unwrap().push((
            combs.len(),
            combs
                .iter()
                .map(|c| buttons[*c].clone())
                .flatten()
                .collect(),
        ));
        return;
    }
    let b = &buttons[index];
    let n = b.iter().fold(0, |acc, n| acc + (1 << ((length - 1) - n)));
    combs.push(index);
    combinations(current ^ n, index + 1, length, buttons, combs, mask);
    combs.pop();
    combinations(current, index + 1, length, buttons, combs, mask);
}
