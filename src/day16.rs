use crate::infra::Problem;
use std::iter::repeat;

pub struct Day16;

impl Problem<String, String, i32, i32> for Day16 {
    fn day() -> u8 {
        16
    }
    fn first(contents: String) -> i32 {
        let mut nums = parse_nums(&contents);
        for _ in 0..100 {
            nums = phase(&nums);
        }
        nums[..8]
            .into_iter()
            .fold(0 as i32, |a, &x| 10 * a + x as i32)
    }
    fn second(contents: String) -> i32 {
        let offset: usize = contents[..7].parse().unwrap();
        let base = parse_nums(&contents);
        let mut nums = base
            .iter()
            .skip(offset % base.len())
            .chain(base.iter().cycle())
            .take(base.len() * 10000 - offset)
            .copied()
            .collect::<Vec<_>>();

        for _ in 0..100 {
            let mut cumsum = 0i32;
            for i in (0..nums.len()).rev() {
                cumsum += nums[i] as i32;
                nums[i] = (cumsum % 10).abs() as i8;
            }
        }
        nums[..8]
            .into_iter()
            .fold(0 as i32, |a, &x| 10 * a + x as i32)
    }
}

fn phase(a: &[i8]) -> Vec<i8> {
    (1..=a.len())
        .map(|n| {
            (a.iter()
                .zip(
                    repeat(0)
                        .take(n)
                        .chain(
                            repeat(1)
                                .take(n)
                                .chain(repeat(0).take(n).chain(repeat(-1).take(n))),
                        )
                        .cycle()
                        .skip(1),
                )
                .map(|(x, y)| (x * y) as i32)
                .sum::<i32>()
                % 10)
                .abs() as i8
        })
        .collect()
}

fn parse_nums(contents: &str) -> Vec<i8> {
    contents
        .trim()
        .split("")
        .filter(|&c| c != "")
        .map(|c| c.parse().unwrap())
        .collect()
}
