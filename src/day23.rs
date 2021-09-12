use crate::computer::{parse_memory, Computer, ComputerState};
use crate::infra::Problem;
use itertools::Itertools;
use std::collections::VecDeque;

pub struct Day23;

impl Problem<String, String, i64, i64, 23> for Day23 {
    fn first(contents: String) -> i64 {
        run_network(&contents, |_, _| true)
    }
    fn second(contents: String) -> i64 {
        run_network(&contents, |a, b| a == b)
    }
}

fn run_network<F: FnMut(Option<(i64, i64)>, Option<(i64, i64)>) -> bool>(
    contents: &str,
    mut is_done: F,
) -> i64 {
    let mem = parse_memory(contents);
    let mut cpus = Vec::new();
    let mut queue = VecDeque::new();
    for i in 0..50 {
        let mut c = Computer::<i64>::from_memory(mem.clone());
        c.run();
        tx_rx(&mut c, i, -1, &mut queue);
        cpus.push(c);
    }

    let mut nat_msg = None;
    let mut last_nat = None;
    loop {
        while let Some((d, x, y)) = queue.pop_front() {
            if d == 255 {
                nat_msg = Some((x, y));
            } else {
                tx_rx(&mut cpus[d as usize], x, y, &mut queue);
            }
        }
        let (x, y) = nat_msg.unwrap();
        if is_done(last_nat, nat_msg) {
            return y;
        }
        tx_rx(&mut cpus[0], x, y, &mut queue);
        last_nat = nat_msg;
    }
}

fn tx_rx(mut c: &mut Computer<i64>, x: i64, y: i64, target: &mut VecDeque<(i64, i64, i64)>) {
    c.run_with_input(x);
    c.run_with_input(y);
    while let ComputerState::HasOutput(_) = c.state {
        target.push_back(c.next_tuple().unwrap());
    }
}
