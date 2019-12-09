use std::collections::{HashMap, VecDeque};

pub struct ComputationResult {
    pub memory: Vec<i128>,
    pub output: Vec<i128>,
}

pub struct Computer {
    ic: usize,
    memory: Vec<i128>,
    vars: HashMap<usize, i128>,
    pub input: VecDeque<i128>,
    pub output: VecDeque<i128>,
    relative_base: i128,
}

impl Computer {
    pub fn from_memory(memory: Vec<i128>) -> Computer {
        Computer {
            ic: 0,
            memory: memory,
            vars: HashMap::new(),
            input: VecDeque::new(),
            output: VecDeque::new(),
            relative_base: 0,
        }
    }

    pub fn run(memory: Vec<i128>, input: Vec<i128>) -> ComputationResult {
        let mut state = Computer::from_memory(memory);
        state.input.extend(&input);
    
        while state.step() == true {}
    
        ComputationResult {
            memory: state.memory,
            output: Vec::from(state.output),
        }
    }

    fn pop(&mut self, parameter_mode: i128) -> usize {
        let ic = self.ic;
        self.ic += 1;
        match parameter_mode % 10 {
            0 => self.read(ic as usize) as usize,
            1 => ic as usize,
            2 => (self.read(ic as usize) + self.relative_base) as usize,
            _ => panic!(),
        }
    }

    fn write(&mut self, pos: usize, val: i128) {
        if pos < self.memory.len() {
            self.memory[pos] = val;
        } else {
            self.vars.insert(pos, val);
        }
    }

    fn read(&self, pos: usize) -> i128 {
        if pos < self.memory.len() {
            self.memory[pos]
        } else {
            self.vars.get(&pos).copied().unwrap_or_default()
        }
    }

    pub fn step(&mut self) -> bool {
        let op = self.memory[self.ic];
        self.ic += 1;
        match op % 100 {
            99 => return false,
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
                let t = self.pop(op / 100);
                let i = self.input.pop_front().unwrap();
                self.write(t, i);
            }
            4 => {
                let p1 = self.pop(op / 100);
                self.output.push_back(self.read(p1));
            }
            5 => {
                let p1 = self.pop(op / 100);
                let p2 = self.pop(op / 1000);
                if self.read(p1) != 0 {
                    self.ic = self.read(p2) as usize;
                }
            }
            6 => {
                let p1 = self.pop(op / 100);
                let p2 = self.pop(op / 1000);
                if self.read(p1) == 0 {
                    self.ic = self.read(p2) as usize;
                }
            }
            7 => {
                let p1 = self.pop(op / 100);
                let p2 = self.pop(op / 1000);
                let t = self.pop(op / 10000);
                self.write(
                    t,
                    if self.read(p1) < self.read(p2) {
                        1
                    } else {
                        0
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
                        1
                    } else {
                        0
                    },
                );
            }
            9 => {
                let p1 = self.pop(op / 100);
                self.relative_base += self.read(p1);
            }
            opcode => panic!(format!("unknown command {}", opcode)),
        };
        true
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
