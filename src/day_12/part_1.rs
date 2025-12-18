use crate::input_util::read_lines;
use std::collections::{HashMap, HashSet};

struct Region {
    w: usize,
    h: usize,
    quantities: Vec<u64>,
}
pub fn solve() {
    let input = read_lines("src/day_12/input.txt");
    let mut presents = Vec::new();
    for i in 0..=5 {
        let mut present = Vec::new();
        for j in 1..=3 {
            let mut s = 0;
            let mut p = 0;
            input[(i * 5) + j].chars().rev().for_each(|c| {
                if c == '#' {
                    s += 1 << p;
                }
                p += 1;
            });
            present.push(s);
        }
        presents.push(present);
    }
    let mut regions = Vec::new();
    for i in 30..input.len() {
        let (size, quantity_str) = input[i].split_once(": ").unwrap();
        let (w, h) = size
            .split_once("x")
            .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
            .unwrap();
        let quantities: Vec<u64> = quantity_str
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        regions.push(Region { w, h, quantities })
    }
    let mut ans = 0;
    for (i,region) in regions.iter().enumerate() {
        let memo = &mut HashSet::new();
        let mut grid = vec![0u64; region.h];
        if bt(&region, &mut vec![0; 6], &mut presents, &mut grid, memo) {
            ans += 1;
        }
        println!("{i}/{}solved! {ans}",regions.len());
    }
    print!("{ans}");
}

fn bt(
    region: &Region,
    used: &mut Vec<u64>,
    presents: &mut Vec<Vec<u64>>,
    grid: &mut Vec<u64>,
    memo: &mut HashSet<(Vec<u64>, Vec<u64>)>,
) -> bool {
    let x1 = (used.clone(), grid.clone());
    if memo.contains(&x1) {
        return false;
    }
    if memo.len() > 250  {
        return false;
    }
    memo.insert(x1);
    for y in 0..=region.h - 3 {
        for x in 0..=region.w - 3 {
            for i in 0..=5 {
                if used[i] < region.quantities[i] {
                    let p = &mut presents[i].clone();
                    'outer: for _ in 0..3 {
                        for j in 0..3 {
                            let n = p[j] << region.w - x - 3;
                            if grid[y + j] & n != 0 {
                                rotate(p);
                                continue 'outer;
                            }
                        }
                        for j in 0..3 {
                            grid[y + j] += p[j] << region.w - x - 3;
                        }
                        used[i] += 1;

                        if bt(region, used, presents, grid, memo) {
                            return true;
                        }
                        used[i] -= 1;
                        for j in 0..3 {
                            grid[y + j] -= p[j] << region.w - x - 3;
                        }
                        rotate(p)
                    }
                }
            }
        }
    }
    /*   for y in 0..region.h {
        println!("{}", &format!("{:#034b}", grid[y])[34 - region.w..34])
    }
    println!();*/
    used == &region.quantities
}
fn rotate(present: &mut Vec<u64>) {
    let mut k = Vec::new();
    let mut b = 4;
    for _ in 0..3 {
        let mut m = 0;
        for j in 0..3 {
            m += (present[j] & b).min(1) << j
        }
        k.push(m);
        b >>= 1;
    }
    for i in 0..3 {
        present[i] = k[i]
    }
}
