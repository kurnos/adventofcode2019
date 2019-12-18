use crate::infra::Problem;
use crate::utils::{Dir, Point2d};
use pathfinding::directed::dijkstra;
use std::collections::HashMap;
use std::convert::TryInto;

type Pos = Point2d<u8>;

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
        let (start, grid) = parse_grid(&contents);
        let all_keys = (1u32 << (b'z' - b'a' + 1)) - 1;

        dijkstra::dijkstra(
            &(start, 0u32),
            |&(p, keys)| successors(&grid, p, keys),
            |&(_, keys)| keys == all_keys,
        )
        .unwrap()
        .1
    }
    fn second(contents: String) -> i32 {
        let (start, mut grid) = parse_grid(&contents);
        grid.remove(&start);
        grid.remove(&(start + Dir::North));
        grid.remove(&(start + Dir::South));
        grid.remove(&(start + Dir::East));
        grid.remove(&(start + Dir::West));

        let all_keys = (1u32 << (b'z' - b'a' + 1)) - 1;

        dijkstra::dijkstra(
            &(
                start + Dir::North + Dir::West,
                start + Dir::North + Dir::East,
                start + Dir::South + Dir::West,
                start + Dir::South + Dir::East,
                0u32,
            ),
            |&(p1, p2, p3, p4, keys)| {
                let mut res = Vec::new();
                res.extend(
                    cost_to_keys(&grid, (p1, keys))
                        .into_iter()
                        .map(|(k, (p, c))| ((p, p2, p3, p4, k), c)),
                );
                res.extend(
                    cost_to_keys(&grid, (p2, keys))
                        .into_iter()
                        .map(|(k, (p, c))| ((p1, p, p3, p4, k), c)),
                );
                res.extend(
                    cost_to_keys(&grid, (p3, keys))
                        .into_iter()
                        .map(|(k, (p, c))| ((p1, p2, p, p4, k), c)),
                );
                res.extend(
                    cost_to_keys(&grid, (p4, keys))
                        .into_iter()
                        .map(|(k, (p, c))| ((p1, p2, p3, p, k), c)),
                );
                res
            },
            |&(_, _, _, _, keys)| keys == all_keys,
        )
        .unwrap()
        .1
    }
}

fn cost_to_keys(grid: &HashMap<Pos, S>, start: (Pos, u32)) -> HashMap<u32, (Pos, i32)> {
    let mut res = HashMap::new();
    for (_, ((p, k), c)) in dijkstra::dijkstra_all(&start, |&(p, keys)| successors(&grid, p, keys))
    {
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

trait Grid2d<P, V> {
    fn neighbours<I: IntoIterator<Item = P>>(&self, pos: &P) -> I;
    fn get(&self, pos: &P) -> P;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

fn successors<'a>(
    grid: &'a HashMap<Pos, S>,
    p: Pos,
    keys: u32,
) -> impl 'a + Iterator<Item = ((Pos, u32), i32)> {
    vec![p + Dir::North, p + Dir::East, p + Dir::West, p + Dir::South]
        .into_iter()
        .flat_map(move |t| match grid.get(&t) {
            None => None,
            Some(S::Door(d)) if d & keys == 0 => None,
            Some(S::Key(k)) => Some(((t, keys | k), 1i32)),
            _ => Some(((t, keys), 1i32)),
        })
}

fn parse_grid(contents: &str) -> (Pos, HashMap<Pos, S>) {
    let mut res = HashMap::new();
    let mut start = None;
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            let p = Pos::new(x.try_into().unwrap(), y.try_into().unwrap());
            match c {
                '#' => None,
                '.' => res.insert(p, S::Empty),
                '@' => {
                    start = Some(p);
                    res.insert(p, S::Empty)
                }
                k if k.is_ascii_lowercase() => res.insert(p, S::Key(1 << ((k as u8) - b'a'))),
                d if d.is_ascii_uppercase() => res.insert(p, S::Door(1 << ((d as u8) - b'A'))),
                _ => panic!(),
            };
        }
    }
    (start.unwrap(), res)
}
