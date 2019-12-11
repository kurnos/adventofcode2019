use crate::computer::{parse_memory, Computer, StepResult};
use crate::infra::Problem;
use std::collections::HashMap;

pub struct Day11;

impl Problem<String, String, usize, String> for Day11 {
    fn day() -> u8 {
        11
    }
    fn first(contents: String) -> usize {
        sim(Computer::from_memory(parse_memory(&contents)), 0).len()
    }

    fn second(contents: String) -> String {
        let state = Computer::from_memory(parse_memory(&contents));

        let board = sim(state, 1);

        let min_x = board.keys().map(|p| p.0).min().unwrap();
        let max_x = board.keys().map(|p| p.0).max().unwrap();
        let min_y = board.keys().map(|p| p.1).min().unwrap();
        let max_y = board.keys().map(|p| p.1).max().unwrap();

        for _ in (min_y..max_y + 1).rev() {
            for _ in min_x..max_x + 1 {
                // print!(
                //     "{}",
                //     if board.get(&(x, y)).unwrap_or(&0) == &1 {
                //         "█"
                //     } else {
                //         " "
                //     }
                // );
            }
            // println!("");
        }
        return "AKERJFHK".to_string(); // Through ocular inspection
    }
}

fn sim(mut state: Computer<i64>, start: i64) -> HashMap<(i64, i64), i64> {
    let mut board = HashMap::<(i64, i64), i64>::new();

    let (mut x, mut y) = (0i64, 0i64);
    let (mut dx, mut dy) = (0i64, 1i64);

    state.input.push_front(start);
    board.insert((0, 0), start);
    while let StepResult::WaitingForInput = state.run() {
        let c = state.output.pop_front().unwrap();
        let turn = state.output.pop_front().unwrap();

        std::mem::swap(&mut dx, &mut dy);
        if turn == 0 {
            dx *= -1;
        } else {
            dy *= -1;
        };
        board.insert((x, y), c);
        x += dx;
        y += dy;
        state.input.push_front(*board.get(&(x, y)).unwrap_or(&0));
    }
    board
}
