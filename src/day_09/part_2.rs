use crate::input_util::read_lines;
use std::collections::HashMap;

#[derive(PartialEq)]
struct Tile {
    x: i64,
    y: i64,
}
pub fn solve() {
    let tiles = read_lines("src/day_09/input.txt")
        .iter()
        .map(|s| {
            let (l, r) = s.split_once(",").unwrap();
            Tile {
                x: l.parse().unwrap(),
                y: r.parse().unwrap(),
            }
        })
        .collect::<Vec<Tile>>();
    let w = tiles.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x as usize + 2;
    let h = tiles.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y as usize + 2;
    let mut grid = vec![vec![0u16; w]; h];
    let mut lines = vec![HashMap::new(); 2];
    for i in 0..tiles.len() {
        let a = &tiles[i];
        let b = if i + 1 == tiles.len() {
            &tiles[0]
        } else {
            &tiles[i + 1]
        };
        if a.x == b.x {
            for i in a.y.min(b.y)..=a.y.max(b.y) {
                grid[i as usize][a.x as usize] = if a.y < b.y { 1 } else { 2 };
            }
            if !lines[0].contains_key(&a.x) {
                lines[0].insert(a.x, Vec::new());
            }

            lines[0]
                .get_mut(&a.x)
                .unwrap()
                .push((a.y.min(b.y)..=a.y.max(b.y)));
        } else {
            if !lines[1].contains_key(&a.y) {
                lines[1].insert(b.y, Vec::new());
            }
            lines[1]
                .get_mut(&a.y)
                .unwrap()
                .push((a.x.min(b.x)..=a.x.max(b.x)));
        }
    }

    let mut max = 0;
    for i in 0..tiles.len() {
        println!("{i}/{}", tiles.len());
        let a = &tiles[i];
        'outer: for j in 0..tiles.len() {
            let b = &tiles[j];
            let (c1, c2) = (Tile { y: a.y, x: b.x }, Tile { y: b.y, x: a.x });
            if !inside(&c1, &grid) || !inside(&c2, &grid) {
                continue;
            }
            let w = (a.x.max(b.x) + a.x.min(b.x)) / 2;
            let h = (a.y.max(b.y) + a.y.min(b.y)) / 2;
            if !inside(&Tile { x:w, y:h }, &grid) {
                continue 'outer;
            }

            for x in a.x.min(b.x) + 1..a.x.max(b.x) {
                if let Some(d) = lines[0].get(&x) {
                    for l in d {
                        if (l.end() > &a.y.min(b.y) && l.start() < &a.y.max(b.y)) {
                            continue 'outer;
                        }
                    }
                }
            }
            for y in a.y.min(b.y) + 1..a.y.max(b.y) {
                if let Some(d) = lines[1].get(&y) {
                    for l in d {
                        if (l.end() > &a.x.min(b.x) && l.start() < &a.x.max(b.x)) {
                            continue 'outer;
                        }
                    }
                }
            }
            let h = a.y.abs_diff(b.y) + 1;
            let w = a.x.abs_diff(b.x) + 1;
            let area = w * h;
            max = max.max(area);
        }
    }
    println!("{max}");
}

fn inside(tile: &Tile, grid: &Vec<Vec<u16>>) -> bool {
    let (y, x) = (tile.y as usize, tile.x as usize);
    let mut count = 0;
    let mut inside = false;
    let mut last_seen = 0;
    for i in 0..=x {
        let n = grid[y][i];
        if n != 0 {
            if n == 2 {
                if !inside || last_seen != 2 {
                    inside = true;
                    count += 1;
                }
            } else {
                if inside {
                    if i == x {
                        break;
                    }
                    inside = false;
                    count += 1;
                }
            }
            last_seen = n;
        }
    }
    count % 2 == 1
}
