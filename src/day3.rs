use std::collections::{HashMap, HashSet};

pub fn first(contents: &String) -> i32 {
    let wires = contents
        .trim()
        .lines()
        .map(|p| parse_wire(p).into_iter().collect::<HashSet<_>>())
        .collect::<Vec<_>>();
    wires[0]
        .intersection(&wires[1])
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

pub fn second(contents: &String) -> usize {
    let wires = contents
        .trim()
        .lines()
        .map(|p| {
            parse_wire(p)
                .into_iter()
                .enumerate()
                .map(|(i, val)| (val, i + 1))
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();
    wires[0]
        .keys()
        .collect::<HashSet<_>>()
        .intersection(&wires[1].keys().collect())
        .map(|p| wires[0][p] + wires[1][p])
        .min()
        .unwrap()
}

fn parse_wire(wire: &str) -> Vec<(i32, i32)> {
    wire.split(',').flat_map(|p| {
        std::iter::repeat(match &p[0..1] {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!(),
        })
        .take(p[1..].parse().unwrap())
    })
    .scan((0i32, 0i32), |p, (dx, dy)| {
        *p = (p.0 + dx, p.1 + dy);
        Some(p.clone())
    })
    .collect()
}
