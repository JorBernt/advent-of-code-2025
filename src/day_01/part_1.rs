use std::str::FromStr;
use crate::input_util::read_lines;

pub fn solve() {
    let lines = read_lines("src/day_01/input.txt");
    let mut dial = 50;
    let mut ans = 0;
    for line in lines {
        let n = i32::from_str(&line[1..line.len()]).unwrap() % 100;
        match &line[0..1] {
            "L" => {
                dial -= n;
                if dial < 0 {
                    dial += 100;
                }
            }
            "R" => dial = (dial + n) % 100,
            _ => {}
        }
        if dial == 0 {
            ans += 1;
        }
    }
    println!("{ans}");
}


