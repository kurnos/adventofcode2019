use std::collections::{HashMap, VecDeque};
use std::convert::TryInto;

pub struct ComputationResult<T> {
    pub memory: Vec<T>,
    pub output: Vec<T>,
}

#[derive(Debug)]
pub enum StepResult {
    Terminated,
    Running,
    WaitingForcontentsut,
}

pub struct Computer<T> {
    pub ic: usize,
    pub memory: Vec<T>,
    pub vars: HashMap<usize, T>,
    pub contentsut: VecDeque<T>,
    pub output: VecDeque<T>,
    pub relative_base: T,
}

impl<T> Computer<T>
where
    T: num::Integer,
    T: TryInto<i16>,
    <T as TryInto<i16>>::Error: std::fmt::Debug,
    T: TryInto<usize>,
    <T as TryInto<usize>>::Error: std::fmt::Debug,
    T: std::ops::AddAssign,
    T: std::marker::Copy,
{
    pub fn from_memory(memory: Vec<T>) -> Computer<T> {
        Computer {
            ic: 0,
            memory: memory,
            vars: HashMap::new(),
            contentsut: VecDeque::new(),
            output: VecDeque::new(),
            relative_base: T::zero(),
        }
    }

    pub fn run_from(memory: Vec<T>, contentsut: Vec<T>) -> ComputationResult<T> {
        let mut state = Computer::from_memory(memory);
        state.contentsut.extend(contentsut);
        match state.run() {
            StepResult::Terminated => ComputationResult {
                memory: state.memory,
                output: Vec::from(state.output),
            },
            end_state => panic!("unexpected state {:?}", end_state),
        }
    }

    pub fn run(&mut self) -> StepResult {
        loop {
            match self.step() {
                StepResult::Running => continue,
                stopped => return stopped,
            }
        }
    }

    fn pop(&mut self, parameter_mode: i16) -> usize {
        let ic = self.ic;
        self.ic += 1;
        match parameter_mode % 10 {
            0 => self.read(ic).try_into().unwrap(),
            1 => ic as usize,
            2 => (self.read(ic) + self.relative_base).try_into().unwrap(),
            _ => panic!(),
        }
    }

    fn write(&mut self, pos: usize, val: T) {
        if pos < self.memory.len() {
            self.memory[pos] = val;
        } else {
            self.vars.insert(pos, val);
        }
    }

    fn read(&self, pos: usize) -> T {
        if pos < self.memory.len() {
            self.memory[pos]
        } else {
            self.vars.get(&pos).copied().unwrap_or_else(|| T::zero())
        }
    }

    pub fn step(&mut self) -> StepResult {
        let op: i16 = self.memory[self.ic].try_into().unwrap();
        self.ic += 1;
        match op % 100 {
            99 => return StepResult::Terminated,
            1 => {
                let p1 = self.pop(op / 100);
                let p2 = self.pop(op / 1000);
                let t = self.pop(op / 10000);
                self.write(t, self.read(p1) + self.read(p2));
            }
            2 => {
                let p1 = self.pop(op / 100);
                let p2 = self.pop(op / 1000);
                let t = self.pop(op / 10000);
                self.write(t, self.read(p1) * self.read(p2));
            }
            3 => {
                if let Some(i) = self.contentsut.pop_front() {
                    let t = self.pop(op / 100);
                    self.write(t, i);
                } else {
                    return StepResult::WaitingForcontentsut;
                }
            }
            4 => {
                let p1 = self.pop(op / 100);
                self.output.push_back(self.read(p1));
            }
            5 => {
                let p1 = self.pop(op / 100);
                let p2 = self.pop(op / 1000);
                if !self.read(p1).is_zero() {
                    self.ic = self.read(p2).try_into().unwrap();
                }
            }
            6 => {
                let p1 = self.pop(op / 100);
                let p2 = self.pop(op / 1000);
                if self.read(p1).is_zero() {
                    self.ic = self.read(p2).try_into().unwrap();
                }
            }
            7 => {
                let p1 = self.pop(op / 100);
                let p2 = self.pop(op / 1000);
                let t = self.pop(op / 10000);
                self.write(
                    t,
                    if self.read(p1) < self.read(p2) {
                        T::one()
                    } else {
                        T::zero()
                    },
                );
            }
            8 => {
                let p1 = self.pop(op / 100);
                let p2 = self.pop(op / 1000);
                let t = self.pop(op / 10000);
                self.write(
                    t,
                    if self.read(p1) == self.read(p2) {
                        T::one()
                    } else {
                        T::zero()
                    },
                );
            }
            9 => {
                let p1 = self.pop(op / 100);
                self.relative_base += self.read(p1);
            }
            opcode => panic!(format!("unknown command {}", opcode)),
        };
        StepResult::Running
    }
}

pub fn parse_memory<T>(content: &String) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    content
        .trim()
        .split(',')
        .map(|s| s.parse::<T>().unwrap())
        .collect()
}
