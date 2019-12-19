use crate::infra::Problem;
use crate::utils::Dir;
use itertools::Either::{Left, Right};
use pathfinding::directed::dijkstra;
use rayon::prelude::*;
use std::collections::HashMap;

pub struct Day18;

#[derive(Debug)]
enum S {
    Empty,
    Key(u32),
    Door(u32),
}

impl Problem<String, String, i32, i32> for Day18 {
    fn day() -> u8 {
        18
    }
    fn first(contents: String) -> i32 {
        let g = Grid::new(contents);

        let all_keys = (1u32 << (b'z' - b'a' + 1)) - 1;

        dijkstra::dijkstra(
            &(Pos(g.map.find('@').unwrap()), 0u32),
            |&(p, keys)| successors(&g, p, keys),
            |&(_, keys)| keys == all_keys,
        )
        .unwrap()
        .1
    }
    fn second(contents: String) -> i32 {
        let mut grid = Grid::new(contents);
        let center = grid.map.find('@').unwrap();
        unsafe {
            let bytes = grid.map.as_bytes_mut();
            bytes[center] = b'#';
            bytes[center + 1] = b'#';
            bytes[center - 1] = b'#';
            bytes[center + grid.width] = b'#';
            bytes[center - grid.width] = b'#';
        }

        let all_keys = (1u32 << (b'z' - b'a' + 1)) - 1;

        dijkstra::dijkstra(
            &(
                [
                    Pos(center + 1 + grid.width),
                    Pos(center + 1 - grid.width),
                    Pos(center - 1 + grid.width),
                    Pos(center - 1 - grid.width),
                ],
                0u32,
            ),
            |&(ps, keys)| {
                ps.par_iter()
                    .enumerate()
                    .flat_map(|(i, p): (usize, &Pos)| {
                        cost_to_keys(&grid, (*p, keys))
                            .into_par_iter()
                            .map(move |(k, (p, c))| {
                                let mut r = ps;
                                r[i] = p;
                                ((r, k), c)
                            })
                    })
                    .collect::<Vec<(([Pos; 4], u32), i32)>>()
            },
            |&(_, keys)| keys == all_keys,
        )
        .unwrap()
        .1
    }
}

fn cost_to_keys(grid: &Grid, start: (Pos, u32)) -> HashMap<u32, (Pos, i32)> {
    let mut res = HashMap::new();
    for (_, ((p, k), c)) in dijkstra::dijkstra_all(&start, |&(p, keys)| {
        if keys.count_ones() <= start.1.count_ones() + 1 {
            Left(successors(&grid, p, keys))
        } else {
            Right(std::iter::empty())
        }
    }) {
        let c = c - 1; // -1 ??
        if k > start.1 {
            res.entry(k)
                .and_modify(|v: &mut (Pos, i32)| {
                    if c < v.1 {
                        *v = (p, c);
                    }
                })
                .or_insert((p, c));
        }
    }
    res
}

struct Grid {
    map: String,
    width: usize,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Pos(usize);

impl Grid {
    fn new(map: String) -> Grid {
        let width = map.lines().next().unwrap().len() + 2;
        Grid { map, width }
    }

    fn get(&self, p: Pos) -> Option<S> {
        match self.map[p.0..=p.0].chars().next().unwrap() {
            '.' | '@' => Some(S::Empty),
            k if k.is_ascii_lowercase() => Some(S::Key(1 << ((k as u8) - b'a'))),
            d if d.is_ascii_uppercase() => Some(S::Door(1 << ((d as u8) - b'A'))),
            _ => None,
        }
    }

    fn neighbours<'a>(&'a self, pos: Pos) -> impl 'a + Iterator<Item = (Pos, S)> {
        vec![Dir::North, Dir::East, Dir::West, Dir::South]
            .into_iter()
            .map(move |d| self.advance(pos, d))
            .flat_map(move |p| self.get(p).map(|s| (p, s)))
    }

    fn advance(&self, pos: Pos, dir: Dir) -> Pos {
        match dir {
            Dir::West => Pos(pos.0 - 1),
            Dir::East => Pos(pos.0 + 1),
            Dir::North => Pos(pos.0 - self.width),
            Dir::South => Pos(pos.0 + self.width),
        }
    }
}

fn successors<'a>(
    grid: &'a Grid,
    p: Pos,
    keys: u32,
) -> impl 'a + Iterator<Item = ((Pos, u32), i32)> {
    grid.neighbours(p).flat_map(move |(t, s)| match s {
        S::Door(d) if d & keys == 0 => None,
        S::Key(k) => Some(((t, keys | k), 1i32)),
        _ => Some(((t, keys), 1i32)),
    })
}
