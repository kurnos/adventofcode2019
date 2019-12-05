pub struct ComputationResult {
    pub memory: Vec<i32>,
    pub output: Vec<i32>,
}

pub fn parse_memory(content: &String) -> Vec<i32> {
    content
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

pub fn run(mut memory: Vec<i32>, input: Vec<i32>) -> ComputationResult {
    let mut out = Vec::new();
    let mut inp = input.into_iter();
    let mut next_ic = Some(0);
    while let Some(ic) = next_ic {
        next_ic = run_instruction(ic, &mut memory, &mut inp, &mut out);
    }
    ComputationResult {
        memory: memory,
        output: out,
    }
}

fn run_instruction<T: Iterator<Item = i32>>(
    ic: usize,
    memory: &mut [i32],
    input: &mut T,
    output: &mut Vec<i32>,
) -> Option<usize> {
    match memory[ic] % 100 {
        99 => None,
        1 => {
            let (p1, p2) = params2(ic, memory);
            memory[memory[ic + 3] as usize] = p1 + p2;
            Some(ic + 4)
        }
        2 => {
            let (p1, p2) = params2(ic, memory);
            memory[memory[ic + 3] as usize] = p1 * p2;
            Some(ic + 4)
        }
        3 => {
            memory[memory[ic + 1] as usize] = input.next().expect("Unexpected end of input");
            Some(ic + 2)
        }
        4 => {
            output.push(params1(ic, memory));
            Some(ic + 2)
        }
        5 => {
            let (p1, p2) = params2(ic, memory);
            if p1 != 0 {
                Some(p2 as usize)
            } else {
                Some(ic + 3)
            }
        }
        6 => {
            let (p1, p2) = params2(ic, memory);
            if p1 == 0 {
                Some(p2 as usize)
            } else {
                Some(ic + 3)
            }
        }
        7 => {
            let (p1, p2) = params2(ic, memory);
            memory[memory[ic + 3] as usize] = if p1 < p2 { 1 } else { 0 };
            Some(ic + 4)
        }
        8 => {
            let (p1, p2) = params2(ic, memory);
            memory[memory[ic + 3] as usize] = if p1 == p2 { 1 } else { 0 };
            Some(ic + 4)
        }
        opcode => panic!(format!("unknown command {}", opcode)),
    }
}

fn params1(ic: usize, memory: &[i32]) -> i32 {
    if (memory[ic] / 100) % 10 == 0 {
        memory[memory[ic + 1 as usize] as usize]
    } else {
        memory[ic + 1 as usize]
    }
}

fn params2(ic: usize, memory: &[i32]) -> (i32, i32) {
    (
        if (memory[ic] / 100) % 10 == 0 {
            (memory[memory[ic + 1 as usize] as usize])
        } else {
            (memory[ic + 1 as usize])
        },
        if (memory[ic] / 1000) % 10 == 0 {
            (memory[memory[ic + 2 as usize] as usize])
        } else {
            (memory[ic + 2 as usize])
        },
    )
}
