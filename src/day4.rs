pub fn first(low: u32, high: u32) -> u32 {
    (low..high).filter(|&x| is_valid1(x)).count() as u32
}

fn decending_counts(mut n: u32) -> Option<[u8; 10]> {
    let mut counts: [u8; 10] = [0; 10];
    let mut last = 9;
    for _ in 0..6 {
        let d = n % 10;
        if d > last {
            return None;
        }
        last = d;
        counts[d as usize] += 1;
        n = n / 10;
    }
    Some(counts)
}

fn is_valid1(n: u32) -> bool {
    if let Some(counts) = decending_counts(n) {
        for &c in &counts {
            if c >= 2 {
                return true;
            }
        }
    }
    false
}

pub fn second(low: u32, high: u32) -> u32 {
    (low..high).filter(|&x| is_valid2(x)).count() as u32
}

fn is_valid2(n: u32) -> bool {
    if let Some(counts) = decending_counts(n) {
        for &c in &counts {
            if c == 2 {
                return true;
            }
        }
    }
    false
}
