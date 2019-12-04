use std::cmp::min;

#[derive(Debug)]
enum Seg {
    H { y: i32, x0: i32, x1: i32 },
    V { x: i32, y0: i32, y1: i32 },
}

pub fn first(contents: &String) -> i32 {
    let wires = contents.lines().map(|p| parse_wire(p)).collect::<Vec<_>>();
    let mut res = std::i32::MAX;
    for (_, s1) in &wires[0] {
        for (_, s2) in &wires[1] {
            if intersects(s1, s2) {
                let n = match (s1, s2) {
                    (Seg::V { x, y0: _, y1: _ }, Seg::H { y, x0: _, x1: _ })
                    | (Seg::H { y, x0: _, x1: _ }, Seg::V { x, y0: _, y1: _ }) => x.abs() + y.abs(),
                    (
                        Seg::V {
                            x,
                            y0: y0a,
                            y1: y1a,
                        },
                        Seg::V {
                            x: _,
                            y0: y0b,
                            y1: y1b,
                        },
                    )
                    | (
                        Seg::H {
                            y: x,
                            x0: y0a,
                            x1: y1a,
                        },
                        Seg::H {
                            y: _,
                            x0: y0b,
                            x1: y1b,
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

pub fn second(contents: &String) -> i32 {
    let wires = contents.lines().map(|p| parse_wire(p)).collect::<Vec<_>>();
    let mut res = std::i32::MAX;
    for (d1, s1) in &wires[0] {
        for (d2, s2) in &wires[1] {
            if intersects(s1, s2) {
                let n = match (s1, s2) {
                    (Seg::V { x, y0, y1: _ }, Seg::H { y, x0, x1: _ })
                    | (Seg::H { y, x0, x1: _ }, Seg::V { x, y0, y1: _ }) => {
                        d1 + d2 + (x - x0).abs() + (y - y0).abs()
                    }
                    (
                        Seg::V {
                            x: _,
                            y0: a0,
                            y1: _,
                        },
                        Seg::V {
                            x: _,
                            y0: b0,
                            y1: _,
                        },
                    )
                    | (
                        Seg::H {
                            y: _,
                            x0: a0,
                            x1: _,
                        },
                        Seg::H {
                            y: _,
                            x0: b0,
                            x1: _,
                        },
                    ) => d1 + d2 + (a0 - b0).abs(),
                };
                if n > 0 && n < res {
                    res = n;
                }
            }
        }
    }
    res
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
        .scan((0i32, 0i32, 0i32), |state, p| {
            let (start, x, y) = *state;
            let d = &p[1..].parse::<i32>().unwrap();
            Some((
                start,
                match &p[0..1] {
                    "U" => {
                        *state = (start + d, x, y + d);
                        Seg::V {
                            x: x,
                            y0: y,
                            y1: y + d,
                        }
                    }
                    "D" => {
                        *state = (start + d, x, y - d);
                        Seg::V {
                            x: x,
                            y0: y,
                            y1: y - d,
                        }
                    }
                    "L" => {
                        *state = (start + d, x - d, y);
                        Seg::H {
                            y: y,
                            x0: x,
                            x1: x - d,
                        }
                    }
                    "R" => {
                        *state = (start + d, x + d, y);
                        Seg::H {
                            y: y,
                            x0: x,
                            x1: x + d,
                        }
                    }
                    _ => panic!(),
                },
            ))
        })
        .collect()
}
