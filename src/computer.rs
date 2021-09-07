use std::convert::TryInto;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum ComputerState<T> {
    NotYetStarted,
    Terminated,
    WaitingForInput,
    HasOutput(T),
}

pub struct Computer<T> {
    pub ic: usize,
    pub state: ComputerState<T>,
    pub input_pos: Option<usize>,
    pub memory: Vec<T>,
    pub relative_base: T,
}

impl<T> Computer<T>
where
    T: num::Integer,
    T: TryInto<i16>,
    <T as TryInto<i16>>::Error: Debug,
    T: TryInto<usize>,
    <T as TryInto<usize>>::Error: Debug,
    T: std::ops::AddAssign,
    T: std::marker::Copy,
{
    pub fn from_str(content: &str) -> Computer<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: Debug,
    {
        Computer::from_memory(parse_memory(content))
    }

    pub fn from_memory(memory: Vec<T>) -> Computer<T> {
        Computer {
            ic: 0,
            state: ComputerState::NotYetStarted,
            memory,
            input_pos: None,
            relative_base: T::zero(),
        }
    }
    pub fn is_terminated(&self) -> bool {
        matches!(self.state, ComputerState::Terminated)
    }

    pub fn run_through(&mut self, mut input: Vec<T>) -> Vec<T> {
        input.reverse();
        let mut res = Vec::new();
        loop {
            match self.state {
                ComputerState::NotYetStarted => {
                    self.run();
                }
                ComputerState::HasOutput(x) => {
                    res.push(x);
                    self.run();
                }
                ComputerState::WaitingForInput => {
                    self.run_with_input(input.pop().unwrap());
                }
                ComputerState::Terminated => break,
            };
        }
        res
    }

    pub fn run(&mut self) -> ComputerState<T> {
        match self.state {
            ComputerState::NotYetStarted | ComputerState::HasOutput(_) => self.exec(),
            _ => panic!(
                "Invalid state {}",
                match self.state {
                    ComputerState::WaitingForInput => "WaitingForInput",
                    ComputerState::Terminated => "Terminated",
                    _ => "",
                }
            ),
        }
    }

    pub fn run_with_input(&mut self, input: T) -> ComputerState<T> {
        match self.state {
            ComputerState::WaitingForInput => {
                self.write(self.input_pos.unwrap(), input);
                self.exec()
            }
            _ => panic!(
                "Invalid state {}",
                match self.state {
                    ComputerState::HasOutput(_) => "Output",
                    ComputerState::Terminated => "Terminated",
                    _ => "",
                }
            ),
        }
    }

    fn exec(&mut self) -> ComputerState<T> {
        loop {
            let op: i16 = self.memory[self.ic].try_into().unwrap();
            self.ic += 1;

            match op % 100 {
                99 => {
                    self.state = ComputerState::Terminated;
                    break;
                }
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
                    self.input_pos = t.try_into().unwrap();
                    self.state = ComputerState::WaitingForInput;
                    break;
                }
                4 => {
                    let p1 = self.pop(op / 100);
                    self.state = ComputerState::HasOutput(self.read(p1));
                    break;
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
                opcode => panic!("unknown command {}", opcode),
            };
        }
        self.state
    }

    fn pop(&mut self, parameter_mode: i16) -> usize {
        let ic = self.ic;
        self.ic += 1;
        match parameter_mode % 10 {
            0 => self.read(ic).try_into().unwrap(),
            1 => ic,
            2 => (self.read(ic) + self.relative_base).try_into().unwrap(),
            _ => panic!(),
        }
    }

    fn write(&mut self, pos: usize, val: T) {
        while pos >= self.memory.len() {
            self.memory.push(T::zero());
        }
        self.memory[pos] = val;
    }

    fn read(&self, pos: usize) -> T {
        if pos < self.memory.len() {
            self.memory[pos]
        } else {
            T::zero()
        }
    }
}

impl<'a, T> Iterator for &mut Computer<T>
where
    T: num::Integer,
    T: TryInto<i16>,
    <T as TryInto<i16>>::Error: Debug,
    T: TryInto<usize>,
    <T as TryInto<usize>>::Error: Debug,
    T: std::ops::AddAssign,
    T: std::marker::Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            ComputerState::NotYetStarted => {
                self.exec();
                self.next()
            }
            ComputerState::HasOutput(x) => {
                self.exec();
                Some(x)
            }
            ComputerState::Terminated => None,
            ComputerState::WaitingForInput => None,
        }
    }
}

pub fn parse_memory<T>(content: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: Debug,
{
    content
        .trim()
        .split(',')
        .map(|s| s.parse::<T>().unwrap())
        .collect()
}
