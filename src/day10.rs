use crate::infra::Problem;
use num::integer::gcd;
use rayon::prelude::*;

use bit_set::BitSet;
use std::collections::HashMap;

pub struct Day10;

const N: usize = 200;

impl Problem<String, String, usize, i32, 10> for Day10 {
    fn first(contents: String) -> usize {
        let asteroids = parse_asteroids(&contents);

        (0..asteroids.len())
            .into_par_iter()
            .map(|i| deltas(asteroids[i], &asteroids[..i], &asteroids[i + 1..]))
            .max()
            .unwrap()
    }
    fn second(contents: String) -> i32 {
        let (from_x, from_y) = (19, 23);
        let asteroids = parse_asteroids(&contents);
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
        100 * (x as i32) + (y as i32)
    }
}

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

fn deltas((x0, y0): (i8, i8), before: &[(i8, i8)], after: &[(i8, i8)]) -> usize {
    let mut seen = BitSet::new();
    for (x, y) in before.iter().chain(after.iter()) {
        let d = gcd(x - x0, y - y0);
        let (dx, dy) = ((x - x0) / d, (y - y0) / d);
        seen.insert((((35 + dx) as usize) << 6) + (dy + 35) as usize);
    }
    seen.len()
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
