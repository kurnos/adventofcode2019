use crate::infra::Problem;
use crate::utils::Dir;
use pathfinding::directed::dijkstra;
use pathfinding::utils::absdiff;
use std::cmp::max;
use std::collections::HashMap;

pub struct Day20;

const PORTAL_ZZ: u16 = 2525;
const PORTAL_AA: u16 = 0;

impl Problem<String, String, i32, i32> for Day20 {
    fn day() -> u8 {
        20
    }
    fn first(contents: String) -> i32 {
        assert_eq!(PORTAL_ZZ, s_to_portal("ZZ"));
        assert_eq!(PORTAL_AA, s_to_portal("AA"));
        let maze = Maze::new(contents);
        let start = maze.find_outer_portal(PORTAL_AA).unwrap();
        let end = maze.find_outer_portal(PORTAL_ZZ).unwrap();

        dijkstra::dijkstra(&start, |&x| successors(&maze, x), |&x| x == end)
            .unwrap()
            .1
            - 2
    }
    fn second(contents: String) -> i32 {
        let maze = Maze::new(contents);
        let start = maze.find_outer_portal(PORTAL_AA).unwrap();
        let end = maze.find_outer_portal(PORTAL_ZZ).unwrap();

        dijkstra::dijkstra(&(start, 0), |&x| successors2(&maze, x), |&(x, _)| x == end)
            .unwrap()
            .1
            - 2
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Pos(usize);

#[derive(Debug)]
struct Maze {
    map: String,
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

fn s_to_portal(s: &str) -> u16 {
    let a = s.as_bytes()[0] - b'A';
    let b = s.as_bytes()[1] - b'A';
    (a as u16) * 100 + b as u16
}

fn successors<'a>(grid: &'a Maze, p: Pos) -> impl 'a + Iterator<Item = (Pos, i32)> {
    grid.neighbours(p).flat_map(move |(t, s)| match s {
        S::Empty => Some((t, 1i32)),
        S::OuterPortal(PORTAL_ZZ) => Some((t, 1i32)),
        S::InnerPortal(p) => grid.find_outer_portal(p).map(|pos| (pos, 0i32)),
        S::OuterPortal(p) => grid.find_inner_portal(p).map(|pos| (pos, 0i32)),
    })
}

fn successors2<'a>(
    grid: &'a Maze,
    (p, level): (Pos, i16),
) -> impl 'a + Iterator<Item = ((Pos, i16), i32)> {
    grid.neighbours(p).flat_map(move |(t, s)| match s {
        S::Empty => Some(((t, level), 1i32)),
        S::OuterPortal(PORTAL_ZZ) if level == 0 => Some(((t, level), 1i32)),
        S::InnerPortal(p) => grid
            .find_outer_portal(p)
            .map(|pos| ((pos, level + 1), 0i32)),
        S::OuterPortal(p) if level > 0 => grid
            .find_inner_portal(p)
            .map(|pos| ((pos, level - 1), 0i32)),
        _ => None,
    })
}

impl Maze {
    fn new(map: String) -> Maze {
        let width = map.lines().next().unwrap().len() + 2;
        let height = map.lines().filter(|s| !s.trim().is_empty()).count();
        let mut res = Maze {
            map,
            width,
            height,
            inner_portals: HashMap::new(),
            outer_portals: HashMap::new(),
        };
        for i in 0..res.map.len() {
            match res.get(Pos(i)) {
                Some(S::InnerPortal(p)) => res.inner_portals.insert(p, Pos(i)),
                Some(S::OuterPortal(p)) => res.outer_portals.insert(p, Pos(i)),
                _ => None,
            };
        }
        res
    }

    fn find_inner_portal(&self, p: u16) -> Option<Pos> {
        self.inner_portals.get(&p).cloned()
    }

    fn find_outer_portal(&self, p: u16) -> Option<Pos> {
        self.outer_portals.get(&p).cloned()
    }

    #[allow(clippy::many_single_char_names)]
    fn get(&self, p: Pos) -> Option<S> {
        match self.map.as_bytes()[p.0] {
            b'.' => Some(S::Empty),
            c if c.is_ascii_uppercase() => {
                let n = self
                    .try_advance(p, Dir::North)
                    .map(|x| self.map.as_bytes()[x.0]);
                let e = self
                    .try_advance(p, Dir::East)
                    .map(|x| self.map.as_bytes()[x.0]);
                let w = self
                    .try_advance(p, Dir::West)
                    .map(|x| self.map.as_bytes()[x.0]);
                let s = self
                    .try_advance(p, Dir::South)
                    .map(|x| self.map.as_bytes()[x.0]);
                let a = [n, w, Some(c), e, s]
                    .iter()
                    .flatten()
                    .filter(|c| c.is_ascii_uppercase())
                    .map(|&b| b as char)
                    .collect::<String>();

                let (x, y) = (p.0 % self.width, p.0 / self.width);
                let (xc, yc) = (self.width / 2, self.height / 2);
                if let Some(b'.') = e {
                    if max(absdiff(x, xc), absdiff(y, yc)) > xc / 2 {
                        Some(S::OuterPortal(s_to_portal(&a)))
                    } else {
                        Some(S::InnerPortal(s_to_portal(&a)))
                    }
                } else if let Some(b'.') = w {
                    if max(absdiff(x, xc), absdiff(y, yc)) > xc / 2 {
                        Some(S::OuterPortal(s_to_portal(&a)))
                    } else {
                        Some(S::InnerPortal(s_to_portal(&a)))
                    }
                } else if let Some(b'.') = s {
                    if max(absdiff(x, xc), absdiff(y, yc)) > xc / 2 {
                        Some(S::OuterPortal(s_to_portal(&a)))
                    } else {
                        Some(S::InnerPortal(s_to_portal(&a)))
                    }
                } else if let Some(b'.') = n {
                    if max(absdiff(x, xc), absdiff(y, yc)) > xc / 2 {
                        Some(S::OuterPortal(s_to_portal(&a)))
                    } else {
                        Some(S::InnerPortal(s_to_portal(&a)))
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn try_advance(&self, pos: Pos, dir: Dir) -> Option<Pos> {
        match dir {
            Dir::West if pos.0 >= 1 => Some(Pos(pos.0 - 1)),
            Dir::East => Some(Pos(pos.0 + 1)),
            Dir::North if pos.0 >= self.width => Some(Pos(pos.0 - self.width)),
            Dir::South if pos.0 + self.width <= self.map.len() => Some(Pos(pos.0 + self.width)),
            _ => None,
        }
    }

    fn advance(&self, pos: Pos, dir: Dir) -> Pos {
        match dir {
            Dir::West => Pos(pos.0 - 1),
            Dir::East => Pos(pos.0 + 1),
            Dir::North => Pos(pos.0 - self.width),
            Dir::South => Pos(pos.0 + self.width),
        }
    }

    fn neighbours<'a>(&'a self, pos: Pos) -> impl 'a + Iterator<Item = (Pos, S)> {
        vec![Dir::North, Dir::East, Dir::West, Dir::South]
            .into_iter()
            .map(move |d| self.advance(pos, d))
            .flat_map(move |p| self.get(p).map(|s| (p, s)))
    }
}
