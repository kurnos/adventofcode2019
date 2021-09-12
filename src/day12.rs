use crate::infra::Problem;
use num::integer::lcm;
use regex::Regex;

pub struct Day12;

impl Problem<String, String, i16, usize, 12> for Day12 {
    fn first(contents: String) -> i16 {
        let (mut xs, mut ys, mut zs) = parse_pos(&contents);
        let (mut vxs, mut vys, mut vzs) = ([0; 4], [0; 4], [0; 4]);

        for _ in 0..1000 {
            step(&mut xs, &mut vxs);
            step(&mut ys, &mut vys);
            step(&mut zs, &mut vzs);
        }

        izip!(&xs, &ys, &zs, &vxs, &vys, &vzs)
            .map(|(x, y, z, vx, vy, vz)| {
                (x.abs() + y.abs() + z.abs()) * (vx.abs() + vy.abs() + vz.abs())
            })
            .sum::<i16>()
    }
    fn second(contents: String) -> usize {
        let (xs, ys, zs) = parse_pos(&contents);

        let mut x: Option<usize> = None;
        let mut y: Option<usize> = None;
        let mut z: Option<usize> = None;
        rayon::scope(|s| {
            s.spawn(|_| x = Some(cycle_length(xs)));
            s.spawn(|_| y = Some(cycle_length(ys)));
            s.spawn(|_| z = Some(cycle_length(zs)));
        });

        lcm(lcm(x.unwrap(), y.unwrap()), z.unwrap())
    }
}

fn step(t: &mut [i16; 4], tv: &mut [i16; 4]) {
    for (i, j) in iproduct!(0..4, 0..4) {
        tv[i] -= (t[i] - t[j]).signum();
    }
    for i in 0..4 {
        t[i] += tv[i];
    }
}

fn cycle_length(ps: [i16; 4]) -> usize {
    let (i0, i1, i2, i3) = (ps[0], ps[1], ps[2], ps[3]);
    let (mut p0, mut p1, mut p2, mut p3) = (ps[0], ps[1], ps[2], ps[3]);
    let (mut v0, mut v1, mut v2, mut v3) = (0, 0, 0, 0);

    let mut i = 0;
    loop {
        i += 1;

        let d01 = p0.wrapping_sub(p1).signum();
        let d02 = p0.wrapping_sub(p2).signum();
        let d03 = p0.wrapping_sub(p3).signum();
        let d12 = p1.wrapping_sub(p2).signum();
        let d13 = p1.wrapping_sub(p3).signum();
        let d23 = p2.wrapping_sub(p3).signum();

        v0 += -d01 - d02 - d03;
        v1 += d01 - d12 - d13;
        v2 += d02 + d12 - d23;
        v3 += d03 + d13 + d23;

        p0 += v0;
        p1 += v1;
        p2 += v2;
        p3 += v3;

        if p0 == i0 && p1 == i1 && p2 == i2 && p3 == i3 && v0 == 0 && v1 == 0 && v2 == 0 && v3 == 0
        {
            return i;
        }
    }
}

fn parse_pos(x: &str) -> ([i16; 4], [i16; 4], [i16; 4]) {
    let re = Regex::new(r"<x=(.*), y=(.*), z=(.*)>").unwrap();
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let mut zs = Vec::new();
    for line in x.trim().lines() {
        let m = re.captures(line).unwrap();
        xs.push(m[1].parse().unwrap());
        ys.push(m[2].parse().unwrap());
        zs.push(m[3].parse().unwrap());
    }
    (
        [xs[0], xs[1], xs[2], xs[3]],
        [ys[0], ys[1], ys[2], ys[3]],
        [zs[0], zs[1], zs[2], zs[3]],
    )
}
