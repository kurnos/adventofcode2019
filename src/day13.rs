use crate::computer::{parse_memory, Computer, StepResult};
use crate::infra::Problem;
use itertools::Itertools;
use std::cmp::Ordering;

pub struct Day13;

impl Problem<String, String, usize, i32> for Day13 {
    fn day() -> u8 {
        13
    }
    fn first(contents: String) -> usize {
        let mut c = Computer::from_memory(parse_memory::<i32>(&contents));
        c.run();
        c.output.iter().tuples().filter(|(_, _, &t)| t == 2).count()
    }
    fn second(contents: String) -> i32 {
        let mut c = Computer::from_memory(parse_memory::<i32>(&contents));
        c.memory[0] = 2;
        let (mut score, mut paddle_x, mut ball_x) = (0, 0, 0);

        loop {
            let r = c.run();

            for (x, _, t) in c.output.iter().tuples() {
                match (x, t) {
                    (-1, s) => score = *s,
                    (x, 3) => paddle_x = *x,
                    (x, 4) => ball_x = *x,
                    _ => {}
                }
            }
            c.output.clear();

            if let StepResult::Terminated = r {
                return score;
            }

            c.input.push_back(match paddle_x.cmp(&ball_x) {
                Ordering::Equal => 0,
                Ordering::Less => 1,
                Ordering::Greater => -1,
            });
        }
    }
}
