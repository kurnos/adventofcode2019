use crate::computer::{Computer, ComputerState};
use crate::infra::Problem;
use itertools::Itertools;
use std::cmp::Ordering;

pub struct Day13;

impl Problem<String, String, usize, i32> for Day13 {
    fn day() -> u8 {
        13
    }
    fn first(contents: String) -> usize {
        Computer::<i32>::from_str(&contents)
            .iter()
            .tuples()
            .filter(|(_, _, t)| *t == 2)
            .count()
    }
    fn second(contents: String) -> i32 {
        let mut c = Computer::<i32>::from_str(&contents);
        c.memory[0] = 2;
        let (mut score, mut paddle_x, mut ball_x) = (0, 0, 0);

        loop {
            for (x, _, t) in c.iter().tuples() {
                match (x, t) {
                    (-1, s) => score = s,
                    (x, 3) => paddle_x = x,
                    (x, 4) => ball_x = x,
                    _ => {}
                }
            }

            if let ComputerState::Terminated = c.state {
                return score;
            }
            c.run_with_input(match paddle_x.cmp(&ball_x) {
                Ordering::Equal => 0,
                Ordering::Less => 1,
                Ordering::Greater => -1,
            });
        }
    }
}
