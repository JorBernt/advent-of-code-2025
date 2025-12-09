use crate::input_util::read_lines;

#[derive(Clone, PartialEq, Eq)]
struct JunctionBox {
    id: usize,
    x: i32,
    y: i32,
    z: i32,
}

impl JunctionBox {
    fn new(id: usize, x: i32, y: i32, z: i32) -> Self {
        JunctionBox { id, x, y, z }
    }
}

struct Connection {
    from: usize,
    to: usize,
    distance: i64,
}
pub fn solve() {
    let mut id = 0;
    let junction_boxes: Vec<JunctionBox> = read_lines("src/day_08/input.txt")
        .iter()
        .map(|s| {
            let cords: Vec<i32> = s.split(",").map(|c| c.parse().unwrap()).collect();
            let junction_box = JunctionBox::new(id, cords[0], cords[1], cords[2]);
            id += 1;
            junction_box
        })
        .collect();
    let mut connections = Vec::new();
    let connections_to_make = 1000;

    for i in 0..junction_boxes.len() {
        for j in (i + 1)..junction_boxes.len() {
            let a = &junction_boxes[i];
            let b = &junction_boxes[j];
            let distance = ((a.x as i64 - b.x as i64).pow(2)
                + (a.y as i64 - b.y as i64).pow(2)
                + (a.z as i64 - b.z as i64).pow(2)).isqrt();
            connections.push(Connection {
                from: a.id,
                to: b.id,
                distance,
            })
        }
    }
    connections.sort_by(|a, b| a.distance.cmp(&b.distance));
    let connections: Vec<&Connection> = connections.iter().take(connections_to_make).collect();
    let len = junction_boxes.len();
    let mut circuit_id = (0..len).collect::<Vec<usize>>();
    let mut circuit_size = vec![1; len];
    for c in &connections {
        let first_circuit = circuit_for(c.from, &mut circuit_id);
        let second_circuit = circuit_for(c.to, &mut circuit_id);
        if first_circuit != second_circuit {
            circuit_id[first_circuit] = second_circuit;
            circuit_size[second_circuit] += circuit_size[first_circuit]
        }
    }
    let mut sizes = circuit_id
        .iter()
        .enumerate()
        .filter_map(|(i, n)| if i == *n { Some(circuit_size[i]) } else { None })
        .collect::<Vec<i32>>();
    sizes.sort_by(|a, b| b.cmp(&a));
    let ans = sizes.iter().take(3).product::<i32>();
    println!("{ans}");
}

fn circuit_for(id: usize, circuit_id: &mut Vec<usize>) -> usize {
    if circuit_id[id] == id {
        return id;
    }
    circuit_id[id] = circuit_for(circuit_id[id], circuit_id);
    circuit_id[id]
}
