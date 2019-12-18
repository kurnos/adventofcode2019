use crate::computer::{parse_memory, Computer};
use crate::infra::Problem;
use crate::utils::{Dir, Point2d};
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day17;

#[derive(Debug)]
enum C {
    L,
    R,
    F,
}

impl Problem<String, String, i16, i64> for Day17 {
    fn day() -> u8 {
        17
    }
    fn first(contents: String) -> i16 {
        let (_, board_map) = get_scaffolding2(&contents);
        board_map
            .iter()
            .filter(|&p| {
                board_map.contains(&(*p + Dir::North))
                    && board_map.contains(&(*p + Dir::South))
                    && board_map.contains(&(*p + Dir::East))
                    && board_map.contains(&(*p + Dir::West))
            })
            .map(|p| p.x * p.y)
            .sum()
    }
    fn second(contents: String) -> i64 {
        let (mut p, scaffolds) = get_scaffolding2(&contents);
        let mut dir = Dir::North;

        let mut commands = Vec::new();
        loop {
            if scaffolds.contains(&(p + dir)) {
                commands.push(C::F);
                p = p + dir;
            } else if scaffolds.contains(&(p + dir.ccw())) {
                commands.push(C::L);
                dir = dir.ccw();
            } else if scaffolds.contains(&(p + dir.cw())) {
                commands.push(C::R);
                dir = dir.cw();
            } else {
                break;
            }
        }

        let (main, fn_a, fn_b, fn_c) = find_program2(commands);

        let mut mem = parse_memory(&contents);
        mem[0] = 2;
        let mut cpu = Computer::<i64>::from_memory(mem);
        let x = cpu.run_through(
            [main, fn_a, fn_b, fn_c, "n\n".to_string()]
                .iter()
                .join("\n")
                .chars()
                .map(|c| c as i64)
                .collect(),
        );

        x.into_iter().last().unwrap()
    }
}

fn get_scaffolding2(contents: &str) -> (Point2d<i16>, HashSet<Point2d<i16>>) {
    let mut cpu = Computer::<i64>::from_str(&contents);
    let x = cpu.run_through(vec![]);
    let board = x.iter().map(|&c| (c as u8) as char).collect::<String>();

    let mut robot = None;
    let mut scaffolds = HashSet::new();
    for (y, line) in board.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            match c {
                '#' => {
                    scaffolds.insert(Point2d::new(x as i16, y as i16));
                }
                '^' => robot = Some(Point2d::new(x as i16, y as i16)),
                _ => (),
            }
        }
    }
    (robot.unwrap(), scaffolds)
}

fn find_program2(commands: Vec<C>) -> (String, String, String, String) {
    let base = commands
        .iter()
        .map(|c| format!("{:?}", c))
        .collect::<String>();

    let a0 = 0;
    for a1 in (a0 + 2)..(base.len() / 2) {
        let a = &base[a0..a1];
        let fn_a = encode(a);
        let after_a = base.replace(a, "A");
        let b0 = after_a.find(|c| !"A".contains(c)).unwrap();
        for (b1, c) in after_a.chars().enumerate().skip(b0 + 2) {
            if c == 'A' {
                break;
            }
            let b = &after_a[b0..=b1];
            let fn_b = encode(b);
            let after_b = after_a.replace(b, "B");
            if let Some(c0) = after_b.find(|c| !"AB".contains(c)) {
                let c1 = after_b[c0..]
                    .find(|c| "AB".contains(c))
                    .map(|i| i + c0)
                    .unwrap_or_else(|| after_b.len());
                let c = &after_b[c0..c1];
                let after_c = after_b.replace(c, "C");
                let fn_main = encode(&after_c);
                let fn_c = encode(c);
                let v = [&fn_main, &fn_a, &fn_b, &fn_c]
                    .iter()
                    .map(|c| c.len())
                    .max()
                    .unwrap();

                if v <= 20 {
                    return (fn_main, fn_a, fn_b, fn_c);
                }
            }
        }
    }
    panic!("Didn't find an encoding!");
}

fn encode(func: &str) -> String {
    let mut f_count = 0;
    let mut res = Vec::new();
    for c in func.chars() {
        if c == 'F' {
            f_count += 1;
        } else if f_count != 0 {
            res.push(format!("{}", f_count));
            res.push(c.to_string());
            f_count = 0;
        } else {
            res.push(c.to_string());
        }
    }
    if f_count != 0 {
        res.push(format!("{}", f_count));
    }
    res.into_iter().join(",")
}
