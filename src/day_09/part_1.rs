use crate::input_util::read_lines;

#[derive(PartialEq)]
struct Tile{
    x:i64,
    y:i64
}
pub fn solve() {
    let tiles = read_lines("src/day_09/input.txt")
        .iter()
        .map(|s|{
            let (l,r)= s.split_once(",").unwrap();
            Tile {
                x: l.parse().unwrap(),
                y: r.parse().unwrap()
            }
        }).collect::<Vec<Tile>>();
    let mut max = 0;
    for i in 0..tiles.len() {
        let a = &tiles[i];
        for j in (i + 1)..tiles.len() {
            let b = &tiles[j];
            let h = a.y.abs_diff(b.y) + 1;
            let w = a.x.abs_diff(b.x) + 1;
            let area = w * h;
            max = max.max(area);
        }
    }
    println!("{max}");
}
