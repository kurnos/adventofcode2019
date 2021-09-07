use crate::infra::Problem;
use crate::utils::Dir;
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
        let all_keys = g.keys().fold(0, |a, x| a | x.1);
        let center = g.find('@').unwrap();
        let neighbours = calc_all_costs(&g, &[center]);

        dijkstra::dijkstra(
            &(center, 0u32),
            |&(p, keys)| {
                neighbours[&p]
                    .iter()
                    .filter(|(_, doors, _, _)| doors | keys == keys)
                    .map(move |(to, _, new_keys, cost)| ((*to, keys | new_keys), *cost))
                    .collect::<Vec<((Pos, u32), i32)>>()
            },
            |&(_, keys)| keys == all_keys,
        )
        .unwrap()
        .1
    }
    fn second(contents: String) -> i32 {
        let mut grid = Grid::new(contents);
        let center = grid.find('@').unwrap();
        unsafe {
            let bytes = grid.map.as_bytes_mut();
            bytes[center.0] = b'#';
            bytes[center.0 + 1] = b'#';
            bytes[center.0 - 1] = b'#';
            bytes[center.0 + grid.width] = b'#';
            bytes[center.0 - grid.width] = b'#';
        }

        let neighbours = calc_all_costs(
            &grid,
            &iproduct!(&[Dir::North, Dir::South], &[Dir::East, Dir::West])
                .map(|(&a, &b)| grid.advance(grid.advance(center, a), b))
                .collect::<Vec<Pos>>(),
        );

        let all_keys = grid.keys().fold(0, |a, x| a | x.1);

        dijkstra::dijkstra(
            &(
                [
                    Pos(center.0 + 1 + grid.width),
                    Pos(center.0 + 1 - grid.width),
                    Pos(center.0 - 1 + grid.width),
                    Pos(center.0 - 1 - grid.width),
                ],
                0u32,
            ),
            |&(ps, keys)| {
                ps.iter()
                    .enumerate()
                    .flat_map(|(i, p)| {
                        neighbours[p]
                            .iter()
                            .filter(|(_, doors, _, _)| doors | keys == keys)
                            .map(move |(to, _, new_keys, cost)| {
                                let mut r = ps;
                                r[i] = *to;
                                ((r, keys | new_keys), *cost)
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

fn calc_all_costs(grid: &Grid, starts: &[Pos]) -> HashMap<Pos, Vec<(Pos, u32, u32, i32)>> {
    let key_positions = grid.keys().map(|x| x.0).collect::<Vec<_>>();
    starts
        .par_iter()
        .chain(key_positions.par_iter())
        .cloned()
        .map(|s| (s, calc_costs(grid, s, &key_positions)))
        .collect()
}

fn calc_costs(grid: &Grid, from: Pos, targets: &[Pos]) -> Vec<(Pos, u32, u32, i32)> {
    let parents = dijkstra::dijkstra_all(&from, |&p| grid.neighbours(p).map(|(p, _)| (p, 1)));
    targets
        .iter()
        .filter_map(|t| {
            parents.get(t).and_then(|p| {
                let mut doors = 0u32;
                let mut keys = 0u32;
                for p_n in dijkstra::build_path(t, &parents).into_iter().skip(1) {
                    match grid.get(p_n) {
                        Some(S::Door(d)) => doors |= d,
                        Some(S::Key(k)) => keys |= k,
                        _ => {}
                    }
                }
                if keys.count_ones() == 1 {
                    Some((*t, doors, keys, p.1))
                } else {
                    None
                }
            })
        })
        .collect()
}

struct Grid {
    map: String,
    width: usize,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Pos(usize);

impl Grid {
    fn new(map: String) -> Grid {
        let (a, b) = {
            let mut lines = map.lines();
            (lines.next().unwrap(), lines.next().unwrap())
        };

        let width = (b.as_ptr() as usize) - (a.as_ptr() as usize);
        Grid { map, width }
    }

    fn keys(&self) -> impl Iterator<Item = (Pos, u32)> + '_ {
        self.map
            .as_bytes()
            .iter()
            .enumerate()
            .filter_map(|(i, b)| match b {
                b'.' | b'@' => None,
                k if k.is_ascii_lowercase() => Some((Pos(i), 1 << (k - b'a'))),
                _ => None,
            })
    }

    fn find(&self, needle: char) -> Option<Pos> {
        self.map.find(needle).map(Pos)
    }

    fn get(&self, p: Pos) -> Option<S> {
        match self.map.as_bytes()[p.0] {
            b'.' | b'@' => Some(S::Empty),
            k if k.is_ascii_lowercase() => Some(S::Key(1 << (k - b'a'))),
            d if d.is_ascii_uppercase() => Some(S::Door(1 << (d - b'A'))),
            _ => None,
        }
    }

    fn neighbours(&self, pos: Pos) -> impl Iterator<Item = (Pos, S)> + '_ {
        (&[Dir::North, Dir::East, Dir::West, Dir::South])
            .iter()
            .map(move |&d| self.advance(pos, d))
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
