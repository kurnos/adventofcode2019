use crate::infra::Problem;

pub struct Day4;

impl Problem<(u32, u32), (u32, u32), u32, u32> for Day4 {
    fn day() -> u8 {
        4
    }
    fn first((low, high): (u32, u32)) -> u32 {
        Ascending::larger_than(low)
            .take_while(|&n| n < high)
            .filter(|&x| is_valid1(x))
            .count() as u32
    }
    fn second((low, high): (u32, u32)) -> u32 {
        Ascending::larger_than(low)
            .take_while(|&n| n < high)
            .filter(|&x| is_valid2(x))
            .count() as u32
    }
}

struct Ascending {
    ns: [u8; 6],
}

impl Ascending {
    fn larger_than(n: u32) -> Ascending {
        let d0 = (n / 100_000) as u8;
        let mut d1 = (n / 10000 % 10) as u8;
        let mut d2 = (n / 1000 % 10) as u8;
        let mut d3 = (n / 100 % 10) as u8;
        let mut d4 = (n / 10 % 10) as u8;
        let mut d5 = (n % 10) as u8;
        if d1 < d0 {
            d1 = d0;
            d2 = d1;
            d3 = d2;
            d4 = d3;
            d5 = d4;
        } else if d2 < d1 {
            d2 = d1;
            d3 = d2;
            d4 = d3;
            d5 = d4;
        } else if d3 < d2 {
            d3 = d2;
            d4 = d3;
            d5 = d4;
        } else if d4 < d3 {
            d4 = d3;
            d5 = d4;
        } else if d5 < d4 {
            d5 = d4;
        }
        Ascending {
            ns: [d0, d1, d2, d3, d4, d5],
        }
    }
}

impl Iterator for Ascending {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ns[0] > 9 {
            return None;
        }
        let val = 100_000 * self.ns[0] as u32
            + 10000 * self.ns[1] as u32
            + 1000 * self.ns[2] as u32
            + 100 * self.ns[3] as u32
            + 10 * self.ns[4] as u32
            + self.ns[5] as u32;

        let mut i = 5;
        self.ns[i] += 1;
        while i > 0 && self.ns[i] > 9 {
            self.ns[i - 1] += 1;
            i -= 1;
        }
        for i in i..5 {
            self.ns[i + 1] = self.ns[i];
        }
        Some(val)
    }
}

fn counts(mut n: u32) -> [u8; 10] {
    let mut counts: [u8; 10] = [0; 10];
    for _ in 0..6 {
        let d = n % 10;
        counts[d as usize] += 1;
        n /= 10;
    }
    counts
}

fn is_valid1(n: u32) -> bool {
    for &c in &counts(n) {
        if c >= 2 {
            return true;
        }
    }
    false
}

fn is_valid2(n: u32) -> bool {
    for &c in &counts(n) {
        if c == 2 {
            return true;
        }
    }
    false
}
