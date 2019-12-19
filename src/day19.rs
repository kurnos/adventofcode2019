use crate::computer::{parse_memory, Computer};
use crate::infra::Problem;
use crate::utils::{Dir, Point2d};
use num::rational::Rational64;

pub struct Day19;

impl Problem<String, String, i64, i64> for Day19 {
    fn day() -> u8 {
        19
    }
    #[allow(clippy::mut_range_bound)]
    fn first(contents: String) -> i64 {
        let mem = parse_memory(&contents);
        let mut s = 0;
        let mut first = 0;
        for y in 0..50 {
            let mut last = 0;
            for x in first..std::cmp::min(2 * y + 1, 50) {
                let mut cpu = Computer::<i64>::from_memory(mem.clone());
                let t = cpu.run_through(vec![x, y])[0];
                if (1, 0) == (last, t) {
                    break;
                } else if (0, 1) == (last, t) {
                    first = x + 1;
                }
                s += t;
                last = t;
            }
        }
        s
    }
    fn second(contents: String) -> i64 {
        let mem = parse_memory(&contents);
        let (low, high) = find_slopes(&mem);
        for y0 in 0.. {
            let y1 = y0 + 100;
            let x0 = (low * y1).to_integer();
            let x1 = (high * y0).to_integer() + 1;
            if x1 - x0 == 100 {
                return y0 + 10000 * x0;
            }
        }
        panic!();
    }
}

fn find_slopes(mem: &[i64]) -> (Rational64, Rational64) {
    let mut low = Rational64::new(0, 1);
    let mut high = Rational64::new(3, 1);

    for y in (1..=5).map(|p| num::pow::pow(10, p)) {
        low = Rational64::new(
            find_next_edge(&mem, Point2d::new((low * y).to_integer(), y), Dir::East).x - 1,
            y,
        );
        high = Rational64::new(
            find_next_edge(&mem, Point2d::new((high * y).to_integer(), y), Dir::West).x + 1,
            y,
        );
    }
    (low, high)
}

fn find_next_edge(mem: &[i64], mut pos: Point2d<i64>, dir: Dir) -> Point2d<i64> {
    let last = trial(mem, pos);
    pos = pos + dir;
    while last == trial(mem, pos) {
        pos = pos + dir;
    }
    pos
}

fn trial(mem: &[i64], p: Point2d<i64>) -> i64 {
    let mut cpu = Computer::<i64>::from_memory(mem.to_owned());
    cpu.run_through(vec![p.x, p.y])[0]
}
