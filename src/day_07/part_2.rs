use crate::input_util::read_lines;
use std::ops::Index;

struct Point {
    y: usize,
    x: usize,
}
impl Point {
    pub fn traverse(&self) -> Point {
        Point {
            y: self.y + 1,
            x: self.x,
        }
    }

    pub fn split(&self) -> (Point, Point) {
        (
            Point {
                y: self.y,
                x: self.x - 1,
            },
            Point {
                y: self.y,
                x: self.x + 1,
            },
        )
    }
}
pub fn solve() {
    let input = read_lines("src/day_07/input.txt");
    let start = input[0].chars().position(|c| c == 'S').unwrap();
    let grid: Vec<Vec<u8>> = input
        .iter()
        .map(|s| s.chars().map(|c| if c == '^' { 1 } else { 0 }).collect())
        .collect();
    let mut dp = vec![vec![0; grid.len() + 1]; grid[0].len() + 1];
    let ans = solve_beams(Point { y: 0, x: start }, &grid, &mut dp) + 1;
    println!("{ans}");
}

fn solve_beams(pos: Point, grid: &Vec<Vec<u8>>, dp: &mut Vec<Vec<u64>>) -> u64 {
    if pos.y >= grid.len() {
        return 0;
    }
    if dp[pos.y][pos.x] != 0 {
        return dp[pos.y][pos.x];
    }
    let s = if grid[pos.y][pos.x] == 0 {
        solve_beams(pos.traverse(), grid, dp)
    } else {
        let (left, right) = pos.split();
        solve_beams(left, grid, dp) + solve_beams(right, grid, dp) + 1
    };
    dp[pos.y][pos.x] = s;
    s
}
