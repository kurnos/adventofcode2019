#![allow(clippy::upper_case_acronyms)]

use crate::computer::Computer;
use crate::infra::Problem;
use itertools::Itertools;
use std::fmt;

pub struct Day21;

#[derive(Debug)]
enum Register {
    A,
    B,
    C,
    D,
    E,
    // F,
    // G,
    H,
    T,
    J,
}

enum Command {
    NOT(Register, Register),
    AND(Register, Register),
    OR(Register, Register),
    WALK,
    RUN,
}

use Command::{AND, NOT, OR, RUN, WALK};
use Register::{A, B, C, D, E, H, J, T};

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NOT(a, b) => write!(f, "NOT {:?} {:?}", a, b),
            AND(a, b) => write!(f, "AND {:?} {:?}", a, b),
            OR(a, b) => write!(f, "OR {:?} {:?}", a, b),
            WALK => writeln!(f, "WALK"),
            RUN => writeln!(f, "RUN"),
        }
    }
}

impl Problem<String, String, i64, i64, 21> for Day21 {
    fn first(contents: String) -> i64 {
        let t = run_program(
            Computer::<i64>::from_str(&contents),
            &[
                NOT(T, T),
                AND(A, T),
                AND(B, T),
                AND(C, T),
                NOT(T, J),
                AND(D, J),
                WALK,
            ],
        );
        t.into_iter().last().unwrap()
    }
    fn second(contents: String) -> i64 {
        let t = run_program(
            Computer::<i64>::from_str(&contents),
            &[
                NOT(T, T),
                AND(A, T),
                AND(B, T),
                AND(C, T),
                NOT(T, T),
                NOT(H, J),
                OR(T, J),
                AND(D, J),
                AND(E, J),
                AND(D, T),
                AND(H, T),
                OR(T, J),
                RUN,
            ],
        );
        t.into_iter().last().unwrap()
    }
}

fn run_program(mut cpu: Computer<i64>, commands: &[Command]) -> Vec<i64> {
    cpu.run_through(
        commands
            .iter()
            .join("\n")
            .as_bytes()
            .iter()
            .map(|&b| b as i64)
            .collect(),
    )
}
