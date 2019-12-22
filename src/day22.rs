use crate::infra::Problem;
use itertools::Itertools;
use mod_exp::mod_exp;

pub struct Day22;

#[derive(Clone, Copy, Debug)]
enum Cmd {
    Cut(i64),
    Rev,
    Deal(i64),
    DealCut(i64, i64),
}

impl Problem<String, String, i64, i64> for Day22 {
    fn day() -> u8 {
        22
    }
    fn first(contents: String) -> i64 {
        let size = 10007;
        forward(&[get_transformation(&contents, size)], 2019, size)
    }
    fn second(contents: String) -> i64 {
        let size = 119_315_717_514_047i64;
        let mut z = 101_741_582_076_661i64;

        let mut ts = Vec::new();
        let mut t = get_transformation(&contents, size);
        let mut s = 1;
        while s <= z {
            ts.push(t);
            s *= 2;
            t = deal_cut(&[t, t], size)[0];
        }
        let mut x = Vec::new();
        while z > 0 {
            let t = ts.pop().unwrap();
            s /= 2;
            if z >= s {
                z -= s;
                x.push(t);
            }
        }
        backward(&x, 2020, size)
    }
}

fn get_transformation(contents: &str, size: i64) -> Cmd {
    let x = deal_cut(
        &rev_rev(&deal_cut(
            &(contents.lines().map(parse).collect::<Vec<_>>()),
            size,
        )),
        size,
    );
    assert_eq!(x.len(), 1);
    x[0]
}

fn deal_cut(cmd: &[Cmd], size: i64) -> Vec<Cmd> {
    let mut res = Vec::new();

    let mut last = None;
    for c1 in cmd.iter().map(|&c| match c {
        Cmd::Cut(cut) => Cmd::DealCut(1, cut),
        Cmd::Deal(inc) => Cmd::DealCut(inc, 0),
        c => c,
    }) {
        if let Some(Cmd::DealCut(inc1, cut1)) = last {
            if let Cmd::DealCut(inc2, cut2) = c1 {
                last = Some(Cmd::DealCut(
                    (((inc1 as i128) * (inc2 as i128)) % size as i128) as i64,
                    (((cut1 as i128) * (inc2 as i128) + cut2 as i128) % size as i128) as i64,
                ));
            } else {
                res.push(Cmd::DealCut(inc1, cut1));
                res.push(c1);
                last = None;
            }
        } else {
            if let Some(c) = last {
                res.push(c)
            }
            last = Some(c1);
        }
    }
    if let Some(c) = last {
        res.push(c)
    }
    res
}

fn rev_rev(cmd: &[Cmd]) -> Vec<Cmd> {
    let mut res = Vec::new();

    for (c0, r0, c1, r1) in cmd.iter().tuples() {
        if let (Cmd::DealCut(inc1, cut1), Cmd::Rev, Cmd::DealCut(inc2, cut2), Cmd::Rev) =
            (c0, r0, c1, r1)
        {
            res.push(Cmd::DealCut(inc1 * inc2, cut1 * inc2 - inc2 - cut2 + 1));
        } else {
            panic!();
        }
    }
    res
}

fn backward(commands: &[Cmd], n: i64, size: i64) -> i64 {
    commands.iter().rev().fold(n, |i, &c| match c {
        Cmd::Cut(n) => cut_r(i, n, size),
        Cmd::Rev => rev_r(i, size),
        Cmd::Deal(n) => deal_r(i, n, size),
        Cmd::DealCut(inc, off) => deal_r(cut_r(i, off, size), inc, size),
    })
}

fn forward(commands: &[Cmd], n: i64, size: i64) -> i64 {
    commands.iter().fold(n, |i, &c| match c {
        Cmd::Cut(n) => cut(i, n, size),
        Cmd::Rev => rev(i, size),
        Cmd::Deal(n) => deal(i, n, size),
        Cmd::DealCut(inc, off) => cut(deal(i, inc, size), off, size),
    })
}

fn cut_r(i: i64, n: i64, size: i64) -> i64 {
    (i + n + size) % size
}

fn rev_r(i: i64, size: i64) -> i64 {
    size - i - 1
}

fn deal_r(i: i64, n: i64, size: i64) -> i64 {
    let inv = mod_exp(n as i128, size as i128 - 2, size as i128);
    ((i as i128 * inv) % size as i128) as i64
}

fn cut(i: i64, n: i64, size: i64) -> i64 {
    (i - n + size) % size
}

fn rev(i: i64, size: i64) -> i64 {
    size - i - 1
}

fn deal(i: i64, n: i64, size: i64) -> i64 {
    ((i as i128 * n as i128) % size as i128) as i64
}

fn parse(cmd: &str) -> Cmd {
    if cmd.starts_with("cut ") {
        Cmd::Cut(cmd.split_whitespace().last().unwrap().parse().unwrap())
    } else if cmd.starts_with("deal into new stack") {
        Cmd::Rev
    } else if cmd.starts_with("deal with increment") {
        Cmd::Deal(cmd.split_whitespace().last().unwrap().parse().unwrap())
    } else {
        panic!();
    }
}
