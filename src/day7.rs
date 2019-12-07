use crate::computer::{parse_memory, run};

pub fn first(contents: &String) -> i32 {
    let mem = parse_memory(contents);

    let mut res = 0;
    for x in 0..5 {
        for y in 0..4 {
            for z in 0..3 {
                for w in 0..2 {
                    for g in 0..1 {
                        let mut numbers = vec![0, 1, 2, 3, 4];
                        let a = run(mem.clone(), vec![numbers.swap_remove(x), 0]).output[0];
                        let b = run(mem.clone(), vec![numbers.swap_remove(y), a]).output[0];
                        let c = run(mem.clone(), vec![numbers.swap_remove(z), b]).output[0];
                        let d = run(mem.clone(), vec![numbers.swap_remove(w), c]).output[0];
                        let e = run(mem.clone(), vec![numbers.swap_remove(g), d]).output[0];
                        if e > res {
                            res = e;
                        }
                    }
                }
            }
        }
    }
    res
}

pub fn second(contents: &String) -> i32 {
    let mem = parse_memory(contents);
    let mut res = 0;
    for x in 0..5 {
        for y in 0..4 {
            for z in 0..3 {
                for w in 0..2 {
                    for g in 0..1 {
                        let mut numbers = vec![5, 6, 7, 8, 9];
                        let x = trial(
                            &mem,
                            [
                                numbers.swap_remove(x),
                                numbers.swap_remove(y),
                                numbers.swap_remove(z),
                                numbers.swap_remove(w),
                                numbers.swap_remove(g),
                            ],
                        );
                        if x > res {
                            res = x;
                        }
                    }
                }
            }
        }
    }
    res
}

fn trial(mem: &Vec<i32>, phases: [i32; 5]) -> i32 {
    let mut thrusters = vec![
        (0, mem.clone()),
        (0, mem.clone()),
        (0, mem.clone()),
        (0, mem.clone()),
        (0, mem.clone()),
    ];

    for i in 0..5 {
        let (ic, mem) = thrusters.get_mut(i).unwrap();
        *ic = run_instruction2(*ic, mem, &mut vec![phases[i]]).unwrap().0;
    }

    let mut data = vec![0];
    let mut asdf;

    for i in (0..5).cycle() {
        asdf = Some((thrusters[i].0, None));
        while let Some((ic, None)) = asdf {
            asdf = run_instruction2(ic, &mut thrusters[i].1, &mut data);
        }
        if let Some((ic, Some(d))) = asdf {
            thrusters[i].0 = ic;
            data.push(d);
        } else {
            break;
        }
    }
    data[0]
}

fn run_instruction2(
    ic: usize,
    memory: &mut [i32],
    input: &mut Vec<i32>,
) -> Option<(usize, Option<i32>)> {
    match memory[ic] % 100 {
        99 => None,
        1 => {
            let (p1, p2) = params2(ic, memory);
            memory[memory[ic + 3] as usize] = p1 + p2;
            Some((ic + 4, None))
        }
        2 => {
            let (p1, p2) = params2(ic, memory);
            memory[memory[ic + 3] as usize] = p1 * p2;
            Some((ic + 4, None))
        }
        3 => {
            memory[memory[ic + 1] as usize] = input.remove(0);
            Some((ic + 2, None))
        }
        4 => Some((ic + 2, Some(params1(ic, memory)))),
        5 => {
            let (p1, p2) = params2(ic, memory);
            if p1 != 0 {
                Some((p2 as usize, None))
            } else {
                Some((ic + 3, None))
            }
        }
        6 => {
            let (p1, p2) = params2(ic, memory);
            if p1 == 0 {
                Some((p2 as usize, None))
            } else {
                Some((ic + 3, None))
            }
        }
        7 => {
            let (p1, p2) = params2(ic, memory);
            memory[memory[ic + 3] as usize] = if p1 < p2 { 1 } else { 0 };
            Some((ic + 4, None))
        }
        8 => {
            let (p1, p2) = params2(ic, memory);
            memory[memory[ic + 3] as usize] = if p1 == p2 { 1 } else { 0 };
            Some((ic + 4, None))
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
