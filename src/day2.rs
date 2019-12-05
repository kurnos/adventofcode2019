use crate::computer::{parse_memory, run};

pub fn first(content: &String) -> i32 {
    let mut memory = parse_memory(content);
    memory[1] = 12;
    memory[2] = 2;
    run(memory, vec![]).memory[0]
}

pub fn second(content: &String) -> i32 {
    let initial = parse_memory(content);
    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = initial.clone();
            memory[1] = noun;
            memory[2] = verb;
            if run(memory, vec![]).memory[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}
