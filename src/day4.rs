struct Ascending {
    ns: [u8; 6],
}

impl Ascending {
    fn larger_than(n: u32) -> Ascending {
        let a = (n / 100000) as u8;
        let mut b = (n / 10000 % 10) as u8;
        let mut c = (n / 1000 % 10) as u8;
        let mut d = (n / 100 % 10) as u8;
        let mut e = (n / 10 % 10) as u8;
        let mut f = (n % 10) as u8;
        if b < a {
            b = a;
            c = b;
            d = c;
            e = d;
            f = e;
        } else if c < b {
            c = b;
            d = c;
            e = d;
            f = e;
        } else if d < c {
            d = c;
            e = d;
            f = e;
        } else if e < d {
            e = d;
            f = e;
        } else if f < e {
            f = e;
        }
        Ascending {
            ns: [a, b, c, d, e, f],
        }
    }
}

impl Iterator for Ascending {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ns[0] > 9 {
            return None;
        }
        let val = 100000 * self.ns[0] as u32
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

pub fn first(low: u32, high: u32) -> u32 {
    Ascending::larger_than(low)
        .take_while(|&n| n < high)
        .filter(|&x| is_valid1(x))
        .count() as u32
}

fn counts(mut n: u32) -> [u8; 10] {
    let mut counts: [u8; 10] = [0; 10];
    for _ in 0..6 {
        let d = n % 10;
        counts[d as usize] += 1;
        n = n / 10;
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

pub fn second(low: u32, high: u32) -> u32 {
    Ascending::larger_than(low)
        .take_while(|&n| n < high)
        .filter(|&x| is_valid2(x))
        .count() as u32
}

fn is_valid2(n: u32) -> bool {
    for &c in &counts(n) {
        if c == 2 {
            return true;
        }
    }
    false
}
