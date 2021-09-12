use crate::computer::{parse_memory, Computer};
use crate::infra::Problem;

pub struct Day2;

impl Problem<String, String, i32, i32, 2> for Day2 {
    fn first(contents: String) -> i32 {
        trial(&parse_memory(&contents), 12, 2)
    }
    fn second(contents: String) -> i32 {
        let target = 19_690_720;
        let initial = parse_memory(&contents);
        let min_noun = (0..100)
            .rev()
            .find(|i| trial(&initial, *i, 99) <= target)
            .unwrap();
        let max_noun = (0..100).find(|i| trial(&initial, *i, 0) >= target).unwrap();
        let verbs = (0..100).collect::<Vec<i32>>();
        for noun in min_noun..=max_noun {
            if let Ok(verb) = verbs.binary_search_by_key(&target, |&v| trial(&initial, noun, v)) {
                return noun * 100 + verb as i32;
            }
        }
        0
    }
}

fn trial(mem: &[i32], noun: i32, verb: i32) -> i32 {
    let mut cpu = Computer::from_memory(mem.to_owned());
    cpu.memory[1] = noun;
    cpu.memory[2] = verb;
    cpu.run();
    cpu.memory[0]
}
