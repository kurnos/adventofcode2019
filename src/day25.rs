use crate::computer::{Computer, ComputerState};
use crate::infra::Problem;
use crate::utils::Dir;
use std::collections::HashSet;

pub struct Day25;

type Cpu = Computer<i64>;

impl Problem<String, String, String, ()> for Day25 {
    fn day() -> u8 {
        25
    }
    fn first(contents: String) -> String {
        let mut cpu = Cpu::from_str(&contents);
        let cpu = &mut cpu;

        let item_black_list = {
            let mut x = HashSet::new();
            x.insert("escape pod");
            x.insert("photons");
            x.insert("molten lava");
            x.insert("infinite loop");
            x.insert("giant electromagnet");
            x
        };

        let mut s = String::new();
        let mut d = Dir::North;
        let mut seen = HashSet::new();
        loop {
            let state = cpu.run();
            match state {
                ComputerState::HasOutput(o) => {
                    s.push((o as u8) as char);
                }
                ComputerState::WaitingForInput => {
                    let (room, dirs, items) = parse_room(&s);
                    for i in items {
                        if item_black_list.contains(i) {
                            continue;
                        }
                        cmd(cpu, &format!("take {}", i));
                    }

                    if room == "Security Checkpoint" {
                        if seen.contains(room) {
                            let items = cmd(cpu, "inv")
                                .lines()
                                .filter(|l| l.starts_with("- "))
                                .map(|l| l.split_at(2).1.trim().to_owned())
                                .collect::<Vec<_>>();

                            for i in 0..(1 << (items.len() + 1)) {
                                for item in items.iter() {
                                    cmd(cpu, &format!("drop {}", item));
                                }
                                for (j, item) in items.iter().enumerate() {
                                    if (1 << j) & i != 0 {
                                        cmd(cpu, &format!("take {}", item));
                                    }
                                }
                                let r = cmd(cpu, "south");
                                if r.contains("Oh, hello!") {
                                    return r
                                        .split("typing ")
                                        .nth(1)
                                        .unwrap()
                                        .split_whitespace()
                                        .next()
                                        .unwrap()
                                        .to_owned();
                                }
                            }
                        }
                        d = d.cw().cw();
                    }
                    if dirs.contains(&d.cw()) {
                        d = d.cw();
                    } else if dirs.contains(&d) {
                    } else if dirs.contains(&d.ccw()) {
                        d = d.ccw();
                    } else {
                        d = d.cw().cw();
                    }
                    walk(cpu, d);
                    seen.insert(room.to_owned());
                    s.clear();
                }
                _ => panic!(),
            }
        }
    }
    fn second(_contents: String) {}
}

fn cmd(cpu: &mut Cpu, s: &str) -> String {
    for c in s.bytes() {
        cpu.run_with_input(c.into());
    }
    cpu.run_with_input(10);
    cpu.map(|i| (i as u8) as char).collect::<String>()
}

fn walk(cpu: &mut Cpu, d: Dir) {
    let s = match d {
        Dir::North => "north\n",
        Dir::South => "south\n",
        Dir::East => "east\n",
        Dir::West => "west\n",
    };
    for c in s.bytes() {
        cpu.run_with_input(c.into());
    }
}

fn parse_room(s: &str) -> (&str, Vec<Dir>, Vec<&str>) {
    let mut doors = Vec::new();
    let room = s.split("==").nth(1).map(|s| s.trim()).unwrap();
    if s.contains("- north") {
        doors.push(Dir::North);
    }
    if s.contains("- west") {
        doors.push(Dir::West);
    }
    if s.contains("- east") {
        doors.push(Dir::East);
    }
    if s.contains("- south") {
        doors.push(Dir::South);
    }
    let items = s
        .split("Items here:\n")
        .nth(1)
        .map(|x| {
            x.lines()
                .take_while(|l| l.starts_with('-'))
                .map(|x| x.split_at(2).1)
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(Vec::new);

    (room, doors, items)
}
