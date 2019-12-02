pub fn first(content: &String) -> u32 {
    let mut x = parse_memory(content);
    x[1] = 12;
    x[2] = 2;
    run(x)
}

pub fn second(content: &String) -> u32 {
    let initial = parse_memory(content);
    for a in 0..100 {
        for b in 0..100 {
            let mut memory = initial.clone();
            memory[1] = a;
            memory[2] = b;
            if run(memory) == 19690720 {
                return 100 * a + b;
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
    let mut pc = 0usize;
    while asdf(pc, &mut memory) {
        pc += 4;
    }
    memory[0]
}

fn asdf(pc: usize, program: &mut [u32]) -> bool {
    if program[pc] == 99 {
        return false;
    } else if program[pc] == 1 {
        program[program[pc + 3] as usize] =
            program[program[pc + 1] as usize] + program[program[pc + 2] as usize];
    } else if program[pc] == 2 {
        program[program[pc + 3] as usize] =
            program[program[pc + 1] as usize] * program[program[pc + 2] as usize];
    } else {
        panic!(format!("unknown command {}", program[pc]));
    }
    true
}
