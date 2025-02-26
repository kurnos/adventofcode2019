use crate::infra::Problem;
use crate::utils::{Dir, Point2d};
use std::cmp::min;

#[derive(Debug)]
enum Seg {
    H { y: i32, x0: i32, x1: i32 },
    V { x: i32, y0: i32, y1: i32 },
}

pub struct Day3;

impl Problem<String, String, i32, i32, 3> for Day3 {
    fn first(contents: String) -> i32 {
        let wires = contents.lines().map(|p| parse_wire(p)).collect::<Vec<_>>();
        let mut res = std::i32::MAX;
        for (_, s1) in &wires[0] {
            for (_, s2) in &wires[1] {
                if intersects(s1, s2) {
                    let n = match (s1, s2) {
                        (Seg::V { x, .. }, Seg::H { y, .. })
                        | (Seg::H { y, .. }, Seg::V { x, .. }) => x.abs() + y.abs(),
                        (
                            Seg::V {
                                x,
                                y0: y0a,
                                y1: y1a,
                            },
                            Seg::V {
                                y0: y0b, y1: y1b, ..
                            },
                        )
                        | (
                            Seg::H {
                                y: x,
                                x0: y0a,
                                x1: y1a,
                            },
                            Seg::H {
                                x0: y0b, x1: y1b, ..
                            },
                        ) => {
                            let mut limits = vec![y0a, y1a, y0b, y1b];
                            limits.sort_unstable();
                            if *limits[1] <= 0 && 0 <= *limits[2] {
                                *x
                            } else {
                                x + min(limits[1].abs(), limits[2].abs())
                            }
                        }
                    };
                    if n > 0 && n < res {
                        res = n;
                    }
                }
            }
        }
        res
    }
    fn second(contents: String) -> i32 {
        let wires = contents.lines().map(|p| parse_wire(p)).collect::<Vec<_>>();
        let mut res = std::i32::MAX;
        for (d1, s1) in &wires[0] {
            for (d2, s2) in &wires[1] {
                if intersects(s1, s2) {
                    let n = match (s1, s2) {
                        (Seg::V { x, y0, .. }, Seg::H { y, x0, .. })
                        | (Seg::H { y, x0, .. }, Seg::V { x, y0, .. }) => {
                            d1 + d2 + (x - x0).abs() + (y - y0).abs()
                        }
                        (Seg::V { y0: a0, .. }, Seg::V { y0: b0, .. })
                        | (Seg::H { x0: a0, .. }, Seg::H { x0: b0, .. }) => {
                            d1 + d2 + (a0 - b0).abs()
                        }
                    };
                    if n > 0 && n < res {
                        res = n;
                    }
                }
            }
        }
        res
    }
}

fn between<T: Ord>(x: T, bound1: T, bound2: T) -> bool {
    if bound1 < bound2 {
        bound1 <= x && x <= bound2
    } else {
        bound2 <= x && x <= bound1
    }
}

fn intersects(a: &Seg, b: &Seg) -> bool {
    match (a, b) {
        (Seg::V { x, y0, y1 }, Seg::H { y, x0, x1 })
        | (Seg::H { y, x0, x1 }, Seg::V { x, y0, y1 })
            if between(x, x0, x1) && between(y, y0, y1) =>
        {
            true
        }
        (
            Seg::V {
                x: da,
                y0: b0a,
                y1: b1a,
            },
            Seg::V {
                x: db,
                y0: b0b,
                y1: b1b,
            },
        )
        | (
            Seg::H {
                y: da,
                x0: b0a,
                x1: b1a,
            },
            Seg::H {
                y: db,
                x0: b0b,
                x1: b1b,
            },
        ) if da == db && (between(b0a, b0b, b1b) || between(b0b, b0a, b1a)) => true,
        _ => false,
    }
}

fn parse_wire(wire: &str) -> Vec<(i32, Seg)> {
    wire.split(',')
        .scan((0i32, Point2d::new(0, 0)), |state, p| {
            let (start, p0) = *state;
            let (head, tail) = p.split_at(1);
            let d = tail.parse::<i32>().unwrap();
            let dir = match head.as_bytes()[0] {
                b'D' => Dir::North,
                b'U' => Dir::South,
                b'R' => Dir::West,
                b'L' => Dir::East,
                _ => unreachable!(),
            };
            *state = (start + d, p0.advance(dir, d));
            Some((
                start,
                match dir {
                    Dir::North | Dir::South => Seg::V {
                        x: p0.x,
                        y0: p0.y,
                        y1: p0.y + d,
                    },
                    Dir::East | Dir::West => Seg::H {
                        x0: p0.x,
                        y: p0.y,
                        x1: p0.x + d,
                    },
                },
            ))
        })
        .collect()
}
