use crate::input_util::read_lines;
use std::collections::HashMap;

#[derive(Debug)]
struct Light {
    diagram: u32,
    buttons: Vec<u32>,
}
pub fn solve() {
    let lines = read_lines("src/day_10/input.txt")
        .iter()
        .map(|l| {
            let diagram_string = &l[1..l.find(|c| c == ']').unwrap()];
            let mut diagram = 0;
            for (i, c) in (diagram_string.char_indices()).rev() {
                if c == '#' {
                    diagram += 1 << i;
                }
            }
            let buttons = l[l.find(|c| c == ']').unwrap() + 1..l.find(|c| c == '{').unwrap() - 1]
                .split_whitespace()
                .map(|s| {
                    s[1..s.len() - 1]
                        .split(",")
                        .map(|d| 1 << d.parse::<u32>().unwrap())
                        .sum::<u32>()
                })
                .collect::<Vec<u32>>();
            Light { diagram, buttons }
        })
        .collect::<Vec<Light>>();
    let ans = lines
        .iter()
        .enumerate()
        .map(|(i, l)| {
            println!("{i}/{}", lines.len());
            calc(l, 0, 0, &mut HashMap::new())
        })
        .collect::<Vec<u32>>();
    print!("{}", ans.iter().sum::<u32>());
}

fn calc(light: &Light, mut current: u32, count: u32, memo: &mut HashMap<u32, u32>) -> u32 {
    if current == light.diagram {
        memo.insert(
            current,
            count.min(*memo.get(&light.diagram).unwrap_or_else(|| &count)),
        );
        return count;
    }
    if memo.contains_key(&light.diagram) {
        if count >= memo[&light.diagram] {
            return u16::MAX as u32;
        }
    }
    if memo.contains_key(&current) {
        if memo[&current] < count {
            return u16::MAX as u32;
        }
    }
    memo.insert(current, count);
    let mut min = u32::MAX;
    for b in &light.buttons {
        min = min.min(calc(light, current ^ b, count + 1, memo))
    }
    min
}
