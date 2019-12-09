use std::collections::{HashMap, VecDeque};

pub struct ComputationResult {
    pub memory: Vec<i128>,
    pub output: Vec<i128>,
}

pub fn parse_memory(content: &String) -> Vec<i128> {
    content
        .trim()
        .split(',')
        .map(|s| s.parse::<i128>().unwrap())
        .collect()
}

pub fn run(mut memory: Vec<i128>, input: Vec<i128>) -> ComputationResult {
    let mut out = VecDeque::new();
    let mut inp = VecDeque::from(input);
    let mut vars = HashMap::new();
    let mut next_ic = Some(0);
    let mut relative_base = 0;
    while let Some(ic) = next_ic {
        next_ic = run_instruction(
            ic,
            &mut memory,
            &mut vars,
            &mut inp,
            &mut out,
            &mut relative_base,
        );
    }
    ComputationResult {
        memory: memory,
        output: Vec::from(out),
    }
}

pub fn run_instruction(
    ic: usize,
    memory: &mut [i128],
    vars: &mut HashMap<usize, i128>,
    input: &mut VecDeque<i128>,
    output: &mut VecDeque<i128>,
    relative_base: &mut i128,
) -> Option<usize> {
    match memory[ic] % 100 {
        99 => None,
        1 => {
            let (p1, p2, t) = params2output(ic, memory, vars, *relative_base);
            //println!("add {}, {}, {}", p1, p2, t);
            write(memory, vars, t, p1 + p2);
            Some(ic + 4)
        }
        2 => {
            let (p1, p2, t) = params2output(ic, memory, vars, *relative_base);
            //println!("mul {}, {}, {}", p1, p2, t);
            write(memory, vars, t, p1 * p2);
            Some(ic + 4)
        }
        3 => {
            let t = paramsoutput(ic, memory, vars, *relative_base);
            write(memory, vars, t, input.pop_front().unwrap());
            Some(ic + 2)
        }
        4 => {
            output.push_back(params1(ic, memory, vars, *relative_base));
            Some(ic + 2)
        }
        5 => {
            let (p1, p2) = params2(ic, memory, vars, *relative_base);
            if p1 != 0 {
                Some(p2 as usize)
            } else {
                Some(ic + 3)
            }
        }
        6 => {
            let (p1, p2) = params2(ic, memory, vars, *relative_base);
            if p1 == 0 {
                Some(p2 as usize)
            } else {
                Some(ic + 3)
            }
        }
        7 => {
            let (p1, p2, t) = params2output(ic, memory, vars, *relative_base);
            write(memory, vars, t, if p1 < p2 { 1 } else { 0 });
            Some(ic + 4)
        }
        8 => {
            let (p1, p2, t) = params2output(ic, memory, vars, *relative_base);
            write(memory, vars, t, if p1 == p2 { 1 } else { 0 });
            Some(ic + 4)
        }
        9 => {
            let p1 = params1(ic, memory, vars, *relative_base);
            *relative_base += p1;
            Some(ic + 2)
        }
        opcode => panic!(format!("unknown command {}", opcode)),
    }
}

fn write(memory: &mut [i128], vars: &mut HashMap<usize, i128>, pos: usize, val: i128) {
    if pos < memory.len() {
        memory[pos] = val;
    } else {
        vars.insert(pos, val);
    }
}

fn read(memory: &[i128], vars: &HashMap<usize, i128>, pos: usize) -> i128 {
    if pos < memory.len() {
        memory[pos]
    } else {
        vars.get(&pos).copied().unwrap_or_default()
    }
}

fn paramsoutput(
    ic: usize,
    memory: &[i128],
    vars: &HashMap<usize, i128>,
    relative_base: i128,
) -> usize {
    match (memory[ic] / 100) % 10 {
        0 => read(memory, vars, ic + 1 as usize) as usize,
        2 => (read(memory, vars, ic + 1 as usize) + relative_base) as usize,
        _ => panic!(),
    }
}

fn params1(ic: usize, memory: &[i128], vars: &HashMap<usize, i128>, relative_base: i128) -> i128 {
    match (memory[ic] / 100) % 10 {
        0 => read(memory, vars, read(memory, vars, ic + 1 as usize) as usize),
        1 => read(memory, vars, ic + 1 as usize),
        2 => read(
            memory,
            vars,
            (read(memory, vars, ic + 1 as usize) + relative_base) as usize,
        ),
        _ => panic!(),
    }
}

// fn params1output(
//     ic: usize,
//     memory: &[i128],
//     vars: &HashMap<usize, i128>,
//     relative_base: i128,
// ) -> (i128, usize) {
//     (
//         match (memory[ic] / 100) % 10 {
//             0 => read(memory, vars, read(memory, vars, ic + 1 as usize) as usize),
//             1 => read(memory, vars, ic + 1 as usize),
//             2 => read(
//                 memory,
//                 vars,
//                 (read(memory, vars, ic + 1 as usize) + relative_base) as usize,
//             ),
//             _ => panic!(),
//         },
//         match (memory[ic] / 1000) % 10 {
//             0 => read(memory, vars, ic + 2 as usize) as usize,
//             2 => (read(memory, vars, ic + 2 as usize) + relative_base) as usize,
//             _ => panic!(),
//         },
//     )
// }

fn params2(
    ic: usize,
    memory: &[i128],
    vars: &HashMap<usize, i128>,
    relative_base: i128,
) -> (i128, i128) {
    (
        match (memory[ic] / 100) % 10 {
            0 => read(memory, vars, read(memory, vars, ic + 1 as usize) as usize),
            1 => read(memory, vars, ic + 1 as usize),
            2 => read(
                memory,
                vars,
                (read(memory, vars, ic + 1 as usize) + relative_base) as usize,
            ),
            _ => panic!(),
        },
        match (memory[ic] / 1000) % 10 {
            0 => read(memory, vars, read(memory, vars, ic + 2 as usize) as usize),
            1 => read(memory, vars, ic + 2 as usize),
            2 => read(
                memory,
                vars,
                (read(memory, vars, ic + 2 as usize) + relative_base) as usize,
            ),
            _ => panic!(),
        },
    )
}

fn params2output(
    ic: usize,
    memory: &[i128],
    vars: &HashMap<usize, i128>,
    relative_base: i128,
) -> (i128, i128, usize) {
    (
        match (memory[ic] / 100) % 10 {
            0 => read(memory, vars, read(memory, vars, ic + 1 as usize) as usize),
            1 => read(memory, vars, ic + 1 as usize),
            2 => read(
                memory,
                vars,
                (read(memory, vars, ic + 1 as usize) + relative_base) as usize,
            ),
            _ => panic!(),
        },
        match (memory[ic] / 1000) % 10 {
            0 => read(memory, vars, read(memory, vars, ic + 2 as usize) as usize),
            1 => read(memory, vars, ic + 2 as usize),
            2 => read(
                memory,
                vars,
                (read(memory, vars, ic + 2 as usize) + relative_base) as usize,
            ),
            _ => panic!(),
        },
        match (memory[ic] / 10000) % 10 {
            0 => read(memory, vars, ic + 3 as usize) as usize,
            2 => (read(memory, vars, ic + 3 as usize) + relative_base) as usize,
            _ => panic!(),
        },
    )
}
