use crate::input_util::read_lines;
use std::str::FromStr;

pub fn solve() {
    let mut input = read_lines("src/day_06/input.txt");
    let max_length = input.iter().map(|s| s.len()).max().unwrap();
    for i in 0..input.len() {
        let l = input[i].len();
        if l < max_length {
            input[i] += &" ".repeat(max_length - l);
        }
    }
    let mut index = max_length;
    let mut last_index = index;
    let mut ans = 0;
    let last_line = input.len() - 1;
    loop {
        if index == 0
            || (0..last_line)
                .into_iter()
                .all(|i| &input[i][index - 1..index] == " ")
        {
            let mut sum = 0;
            for i in (index..last_index).rev() {
                let num: Vec<&str> = (0..last_line)
                    .into_iter()
                    .map(|y| &input[y][i..i + 1])
                    .filter(|s| s != &" ")
                    .collect();
                let m = u128::from_str(&num.join("")).unwrap();
                match &input[last_line][index..index + 1] {
                    "+" => sum += m,
                    "*" => {
                        let n = m;
                        if sum == 0 {
                            sum = n;
                        } else {
                            sum *= n
                        }
                    }
                    &_ => panic!("Invalid operations"),
                }
            }
            ans += sum;
            if index != 0 {
                last_index = index - 1;
            }
        }
        if index == 0 {
            break;
        }
        index -= 1;
    }

    println!("{ans}")
}
