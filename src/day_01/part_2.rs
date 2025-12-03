use crate::input_util::read_lines;
use std::str::FromStr;

pub fn solve() {
    let lines = read_lines("src/day_01/input.txt");
    let mut dial = 50;
    let mut ans = 0;
    for line in lines {
        let n = i32::from_str(&line[1..line.len()]).unwrap();
        match &line[0..1] {
            "L" => {
                ans += ((dial -n) / 100).abs();
                dial -= n % 100;
                if dial < 0 {
                    ans += 1;
                    dial += 100;
                }
            }
            "R" => {
                let times_passed_zero = (dial + n) / 100;
                ans += times_passed_zero;
                dial = (dial + n) % 100
            }
            _ => {}
        }
        if dial == 0 {
            ans += 1;
        }
    }
    println!("{ans}");
}
