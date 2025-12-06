use crate::input_util::read_lines;
use std::str::FromStr;

enum Operation {
    Addition,
    Multiplication
}
pub fn solve() {
    let input = read_lines("src/day_06/input.txt");
    let mut columns: Vec<Vec<u128>> = Vec::new();
    let mut operations: Vec<Operation> = Vec::new();
    for line in input {
        if line.contains("*") {
            for s in line.split_whitespace() {
                match s {
                    "+" => operations.push(Operation::Addition),
                    "*" => operations.push(Operation::Multiplication),
                    &_ => {panic!("Invalid operation: {s}")}
                }
            }
            continue;
        }
        let mut index = 0;
        for n in line.split_whitespace().map(|s| u128::from_str(s).unwrap()) {
            if let Some(col) = columns.get_mut(index) {
                col.push(n);
            } else {
                columns.push(vec![n]);
            }
            index += 1;
        }
    }
    let mut ans = 0;
    for i in 0..columns.len() {
        match operations[i] {
            Operation::Addition => ans += columns[i].iter().sum::<u128>(),
            Operation::Multiplication => ans += columns[i].iter().product::<u128>(),
        }
    }
    println!("{ans}")
}
