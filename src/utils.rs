// use std::collections::HashMap;
// use std::hash::Hash;

pub fn permutations<T: Clone>(mut xs: Vec<T>) -> Vec<Vec<T>> {
    permutations_inner(&mut xs, 0)
}

fn permutations_inner<T: Clone>(xs: &mut Vec<T>, low: usize) -> Vec<Vec<T>> {
    if low + 1 >= xs.len() {
        return vec![xs.clone()];
    } else {
        let mut result = permutations_inner(xs, low + 1);
        for i in (low + 1)..xs.len() {
            xs.swap(low, i);
            result.extend(permutations_inner(xs, low + 1));
            xs.swap(low, i);
        }
        result
    }
}

pub fn bisect<TIn, TOut, F: Fn(TIn) -> TOut>(f: F, mut low: TIn, mut high: TIn, target: TOut) -> TIn
where
    TIn: std::ops::Sub<Output = TIn>
        + std::ops::Add<Output = TIn>
        + std::ops::Shr<Output = TIn>
        + std::cmp::PartialOrd
        + num::One
        + std::marker::Copy,
    TOut: std::cmp::PartialOrd,
{
    if f(high) < f(low) {
        std::mem::swap(&mut high, &mut low);
    }

    while high - low > TIn::one() {
        if f((low + high) >> num::one()) > target {
            high = (low + high) >> num::one();
        } else {
            low = (low + high) >> num::one();
        }
    }
    low
}

// pub fn counts<It, Item>(iter: It) -> HashMap<Item, u32>
// where
//     It: Iterator<Item = Item>,
//     Item: Hash + Eq,
// {
//     let mut res = HashMap::new();
//     for i in iter {
//         *res.entry(i).or_default() += 1;
//     }
//     res
// }
