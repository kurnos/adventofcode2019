use crate::computer::{Computer, ComputerState};
use crate::infra::Problem;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Day11;

impl Problem<String, String, usize, String> for Day11 {
    fn day() -> u8 {
        11
    }
    fn first(contents: String) -> usize {
        paint(Computer::from_str(&contents), 0).len()
    }

    fn second(contents: String) -> String {
        let board = paint(Computer::from_str(&contents), 1);

        let min_x = board.keys().map(|p| p.0).min().unwrap();
        let max_x = board.keys().map(|p| p.0).max().unwrap();
        let min_y = board.keys().map(|p| p.1).min().unwrap();
        let max_y = board.keys().map(|p| p.1).max().unwrap();

        for y in min_y..=max_y {
            println!(
                "{}",
                (min_x..=max_x)
                    .map(|x| match board.get(&(x, y)) {
                        Some(1) => '█',
                        _ => ' ',
                    })
                    .collect::<String>()
            );
        }
        "AKERJFHK".to_string() // Through ocular inspection
    }
}

fn paint(mut vm: Computer<i64>, start: i64) -> HashMap<(i64, i64), i64> {
    let mut board = HashMap::<(i64, i64), i64>::new();

    let (mut x, mut y, mut dir) = (0i64, 0i64, 0i64);

    board.insert((0, 0), start);
    vm.run();
    while let ComputerState::WaitingForInput = vm.state {
        vm.run_with_input(*board.get(&(x, y)).unwrap_or(&0));
        let (c, turn) = vm.iter().next_tuple().unwrap();

        board.insert((x, y), c);
        dir = (dir + 5 - 2 * turn) % 4;
        match dir {
            0 => y -= 1,
            1 => x -= 1,
            2 => y += 1,
            3 => x += 1,
            _ => {}
        };
    }
    board
}
