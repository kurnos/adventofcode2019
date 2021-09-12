use crate::infra::Problem;
use crate::utils::Dir;
use pathfinding::directed::dijkstra;
use pathfinding::utils::absdiff;
use std::cmp::max;
use std::collections::HashMap;

pub struct Day20;

const PORTAL_ZZ: u16 = 2525;
const PORTAL_AA: u16 = 0;

impl Problem<String, String, i32, i32, 20> for Day20 {
    fn first(contents: String) -> i32 {
        assert_eq!(PORTAL_ZZ, b_to_portal(b'Z', b'Z'));
        assert_eq!(PORTAL_AA, b_to_portal(b'A', b'A'));
        let maze = Maze::new(contents);
        let start = maze.find_outer_portal(PORTAL_AA).unwrap();
        let end = maze.find_outer_portal(PORTAL_ZZ).unwrap();

        dijkstra::dijkstra(
            &start,
            |&p| {
                maze.neighbours(p).flat_map(|(p, s)| match s {
                    S::Empty => Some((p, 1i32)),
                    S::OuterPortal(PORTAL_ZZ) => Some((p, 0i32)),
                    S::InnerPortal(n) => maze.find_outer_portal(n).map(|p| (p, 0i32)),
                    S::OuterPortal(n) => maze.find_inner_portal(n).map(|p| (p, 0i32)),
                })
            },
            |&x| x == end,
        )
        .unwrap()
        .1 - 1
    }
    fn second(contents: String) -> i32 {
        let maze = Maze::new(contents);
        let start = maze.find_outer_portal(PORTAL_AA).unwrap();
        let end = maze.find_outer_portal(PORTAL_ZZ).unwrap();

        dijkstra::dijkstra(
            &(start, 0),
            |&(pos, level)| {
                let maze = &maze;
                maze.neighbours(pos).flat_map(move |(t, s)| match s {
                    S::Empty => Some(((t, level), 1i32)),
                    S::OuterPortal(PORTAL_ZZ) if level == 0 => Some(((t, level), 1i32)),
                    S::InnerPortal(p) => maze
                        .find_outer_portal(p)
                        .map(|pos| ((pos, level + 1), 0i32)),
                    S::OuterPortal(p) if level > 0 => maze
                        .find_inner_portal(p)
                        .map(|pos| ((pos, level - 1), 0i32)),
                    _ => None,
                })
            },
            |&(x, _)| x == end,
        )
        .unwrap()
        .1 - 2
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Pos(usize);

#[derive(Debug)]
struct Maze {
    map: Vec<u8>,
    width: usize,
    height: usize,
    inner_portals: HashMap<u16, Pos>,
    outer_portals: HashMap<u16, Pos>,
}

#[derive(Debug, Copy, Clone)]
enum S {
    Empty,
    InnerPortal(u16),
    OuterPortal(u16),
}

fn b_to_portal(a: u8, b: u8) -> u16 {
    let a = a - b'A';
    let b = b - b'A';
    (a as u16) * 100 + b as u16
}

impl Maze {
    fn new(map: String) -> Maze {
        let width = map.find('\n').unwrap() + 1;
        let height = map.lines().filter(|s| !s.trim().is_empty()).count();
        let mut res = Maze {
            map: map.into_bytes(),
            width,
            height,
            inner_portals: HashMap::new(),
            outer_portals: HashMap::new(),
        };
        for i in 0..res.map.len() {
            res.get(Pos(i)).map(|s| match s {
                S::InnerPortal(n) => res.inner_portals.insert(n, Pos(i)),
                S::OuterPortal(n) => res.outer_portals.insert(n, Pos(i)),
                _ => None,
            });
        }
        res
    }

    fn find_inner_portal(&self, p: u16) -> Option<Pos> {
        self.inner_portals.get(&p).cloned()
    }

    fn find_outer_portal(&self, p: u16) -> Option<Pos> {
        self.outer_portals.get(&p).cloned()
    }

    fn get_byte(&self, p: Pos) -> u8 {
        self.map[p.0]
    }

    #[allow(clippy::many_single_char_names)]
    fn get(&self, p: Pos) -> Option<S> {
        match self.map[p.0] {
            b'.' => Some(S::Empty),
            c if c.is_ascii_uppercase() => {
                let (x, y) = (p.0 % self.width, p.0 / self.width);
                let (xc, yc) = (self.width / 2, self.height / 2);
                match (
                    self.try_advance(p, Dir::West).map(&|t| self.get_byte(t)),
                    self.try_advance(p, Dir::East).map(&|t| self.get_byte(t)),
                    self.try_advance(p, Dir::North).map(&|t| self.get_byte(t)),
                    self.try_advance(p, Dir::South).map(&|t| self.get_byte(t)),
                ) {
                    (Some(b'.'), Some(a), _, _) if a.is_ascii_uppercase() => Some((c, a)),
                    (Some(a), Some(b'.'), _, _) if a.is_ascii_uppercase() => Some((a, c)),
                    (_, _, Some(b'.'), Some(a)) if a.is_ascii_uppercase() => Some((c, a)),
                    (_, _, Some(a), Some(b'.')) if a.is_ascii_uppercase() => Some((a, c)),
                    _ => None,
                }
                .map(|(a, b)| b_to_portal(a, b))
                .map(|n| {
                    if max(absdiff(x, xc), absdiff(y, yc)) > xc / 2 {
                        S::OuterPortal(n)
                    } else {
                        S::InnerPortal(n)
                    }
                })
            }
            _ => None,
        }
    }

    fn try_advance(&self, pos: Pos, dir: Dir) -> Option<Pos> {
        match dir {
            Dir::West if pos.0 >= 1 => Some(Pos(pos.0 - 1)),
            Dir::East if pos.0 + 1 < self.map.len() => Some(Pos(pos.0 + 1)),
            Dir::North if pos.0 >= self.width => Some(Pos(pos.0 - self.width)),
            Dir::South if pos.0 + self.width < self.map.len() => Some(Pos(pos.0 + self.width)),
            _ => None,
        }
    }

    fn neighbours(&self, pos: Pos) -> impl Iterator<Item = (Pos, S)> + '_ {
        vec![Dir::North, Dir::East, Dir::West, Dir::South]
            .into_iter()
            .flat_map(move |d| self.try_advance(pos, d))
            .flat_map(move |p| self.get(p).map(|s| (p, s)))
    }
}
