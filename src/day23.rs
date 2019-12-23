use crate::computer::{parse_memory, Computer, ComputerState};
use crate::infra::Problem;
use itertools::Itertools;
use std::collections::VecDeque;

pub struct Day23;

impl Problem<String, String, i64, i64> for Day23 {
    fn day() -> u8 {
        23
    }
    fn first(contents: String) -> i64 {
        run_network(&contents, |_, m| m.is_some()).unwrap().1
    }
    fn second(contents: String) -> i64 {
        let mut last_y = None;
        run_network(&contents, |idle, m| match (idle, m, last_y) {
            (true, Some((_, y)), Some(last)) if last == y => true,
            (true, Some((_, y)), _) => {
                last_y = Some(y);
                false
            }
            _ => false,
        })
        .unwrap()
        .1
    }
}

fn run_network<F: FnMut(bool, Option<(i64, i64)>) -> bool>(
    contents: &str,
    mut is_done: F,
) -> Option<(i64, i64)> {
    let mem = parse_memory(contents);
    let mut cpus = Vec::new();
    let mut queues = Vec::new();
    for i in 0..50 {
        let mut c = Computer::<i64>::from_memory(mem.clone());
        c.run();
        c.run_with_input(i);
        cpus.push(c);
        queues.push(VecDeque::<(i64, i64)>::new());
    }

    let mut nat_msg = None;
    loop {
        let mut idle = true;
        for (i, mut c) in cpus.iter_mut().enumerate() {
            match (i, c.state, queues[i].get(0).cloned()) {
                (_, ComputerState::WaitingForInput, None) => {
                    c.run_with_input(-1);
                }
                (i, ComputerState::WaitingForInput, Some((x, y))) => {
                    c.run_with_input(x);
                    c.run_with_input(y);
                    queues[i].pop_front();
                    idle = false;
                }
                (_, ComputerState::HasOutput(_), _) => {
                    let (d, x, y) = c.next_tuple().unwrap();
                    if d == 255 {
                        nat_msg = Some((x, y));
                    } else {
                        queues[d as usize].push_back((x, y));
                    }
                    idle = false;
                }
                _ => {}
            }
        }
        if is_done(idle, nat_msg) {
            return nat_msg;
        }
        if let (true, Some((x, y))) = (idle, nat_msg) {
            queues[0].push_back((x, y));
        }
    }
}
