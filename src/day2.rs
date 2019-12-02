pub fn first(content: &String) -> u32 {
    let mut x = parse_memory(content);
    x[1] = 12;
    x[2] = 2;
    run(x)
}

pub fn second(content: &String) -> u32 {
    let initial = parse_memory(content);
    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = initial.clone();
            memory[1] = noun;
            memory[2] = verb;
            if run(memory) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

fn parse_memory(content: &String) -> Vec<u32> {
    content
        .trim()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn run(mut memory: Vec<u32>) -> u32 {
    std::iter::successors(Some(0usize), |&ic| run_instruction(ic, &mut memory)).last();
    memory[0]
}

fn run_instruction(ic: usize, memory: &mut [u32]) -> Option<usize> {
    match memory[ic] as usize {
        99 => {
            None
        },
        1 => {
            memory[memory[ic + 3] as usize] =
            memory[memory[ic + 1] as usize] + memory[memory[ic + 2] as usize];
            Some(ic + 4)
        },
        2 => {
            memory[memory[ic + 3] as usize] =
            memory[memory[ic + 1] as usize] * memory[memory[ic + 2] as usize];
            Some(ic + 4)
        },
        opcode => panic!(format!("unknown command {}", memory[opcode]))
    }
}
