use crate::computer::{parse_memory, run};

pub fn first(content: &String) -> i32 {
    trial(&parse_memory(content), 12, 2)
}

pub fn second(content: &String) -> i32 {
    let target = 19690720;
    let initial = parse_memory(content);
    let min_noun = (0..100)
        .rev()
        .find(|i| trial(&initial, *i, 99) <= target)
        .unwrap();
    let max_noun = (0..100).find(|i| trial(&initial, *i, 0) >= target).unwrap();
    let verbs = (0..100).collect::<Vec<i32>>();
    for noun in min_noun..(max_noun + 1) {
        if let Ok(verb) = verbs.binary_search_by_key(&target, |&v| trial(&initial, noun, v)) {
            return noun * 100 + verb as i32;
        }
    }
    0
}

fn trial(mem: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut memory = mem.clone();
    memory[1] = noun;
    memory[2] = verb;
    run(memory, vec![]).memory[0]
}
