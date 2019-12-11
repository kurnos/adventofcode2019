use num::integer::gcd;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

fn parse_asteroids(contents: &str) -> Vec<(i8, i8)> {
    let mut res = Vec::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, b) in line.chars().enumerate() {
            if b == '#' {
                res.push((x as i8, y as i8));
            }
        }
    }
    res
}

fn deltas((x0, y0): (i8, i8), field: &Vec<(i8, i8)>) -> HashSet<(i8, i8)> {
    let mut seen = HashSet::new();
    for (x, y) in field {
        if x0 == *x && y0 == *y {
            continue;
        }
        let d = gcd(x - x0, y - y0);
        seen.insert(((x - x0) / d, (y - y0) / d));
    }
    seen
}

pub fn first(contents: &String) -> usize {
    let asteroids = parse_asteroids(contents);

    return asteroids
        .par_iter()
        .map(|p| deltas(*p, &asteroids).len())
        .max()
        .unwrap();
}

fn deltas_map((x0, y0): (i8, i8), asteroids: Vec<(i8, i8)>) -> HashMap<(i8, i8), (i8, i8, i8)> {
    let mut seen = HashMap::new();
    for (x, y) in asteroids {
        if x0 == x && y0 == y {
            continue;
        }
        let d = gcd(x - x0, y - y0);

        seen.entry(((x - x0) / d, (y - y0) / d))
            .and_modify(|e: &mut (i8, i8, i8)| {
                if d < e.0 {
                    *e = (d, x, y);
                }
            })
            .or_insert((d, x, y));
    }
    seen
}

const N: usize = 200;

pub fn second(contents: &String) -> i32 {
    let (from_x, from_y) = (19, 23);
    let asteroids = parse_asteroids(contents);

    let targets = deltas_map((from_y as i8, from_x as i8), asteroids);

    let mut shooting_order = targets.keys().collect::<Vec<_>>();
    shooting_order.sort_by(|(ax, ay), (bx, by)| {
        (*ax as f32)
            .atan2(*ay as f32)
            .partial_cmp(&(*bx as f32).atan2(*by as f32))
            .unwrap_or(std::cmp::Ordering::Equal)
            .reverse()
    });
    let (_, x, y) = targets[shooting_order[N - 1]];
    return 100 * (x as i32) + (y as i32);
}
