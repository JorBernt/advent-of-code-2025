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
    let mut grid: Vec<Vec<u8>> = input
        .iter()
        .map(|s| s.chars().map(|c| if c == '^' {1} else {0}).collect())
        .collect();
    let ans = solve_beams(Point { y: 0, x: start }, &mut grid);
    println!("{ans}");
}

fn solve_beams(pos: Point, grid: &mut Vec<Vec<u8>>) -> i32 {
    if pos.y >= grid.len() {
        return 0;
    }
    if grid[pos.y][pos.x] == 0 {
        grid[pos.y][pos.x] = 2;
        return solve_beams(pos.traverse(), grid) ;
    } else if grid[pos.y][pos.x] == 1 {
        let (left, right) = pos.split();
        return solve_beams(left, grid)  + solve_beams(right, grid)  + 1
    }
    0
}
