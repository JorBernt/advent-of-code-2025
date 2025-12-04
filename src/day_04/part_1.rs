use crate::input_util::read_lines;
use std::fs::read;
use std::io::read_to_string;

pub fn solve() {
    let grid: Vec<Vec<bool>> = read_lines("src/day_04/input.txt")
        .iter()
        .map(|row| row.chars().map(|c| c == '@').collect())
        .collect();
    let mut ans = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] && valid(y, x, &grid) {
                ans += 1;
            }
        }
    }
    println!("{ans}")
}

fn valid(y: usize, x: usize, grid: &Vec<Vec<bool>>) -> bool {
    let dirs: Vec<(i32,i32)> = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    dirs.iter().filter(|(dy, dx)| {
        let y = y as i32 + dy;
        let x = x as i32 + dx;
        if y < 0 || x < 0 || y as usize >= grid.len() || x as usize >= grid[0].len()  {
            return false
        }
        *grid
            .get(y as usize)
            .unwrap()
            .get(x as usize)
            .unwrap()
    }).count() < 4
}
