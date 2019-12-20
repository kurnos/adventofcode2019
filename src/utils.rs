// use std::collections::HashMap;
// use std::hash::Hash;

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Point2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2d<T> {
    pub fn new(x: T, y: T) -> Point2d<T> {
        Point2d { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dir {
    North,
    West,
    East,
    South,
}

impl Dir {
    pub fn cw(self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }

    pub fn ccw(self) -> Dir {
        match self {
            Dir::East => Dir::North,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
            Dir::North => Dir::West,
        }
    }
}

impl<T> std::ops::Add<Dir> for Point2d<T>
where
    T: std::ops::Sub<Output = T>,
    T: std::ops::Add<Output = T>,
    T: num::One,
{
    type Output = Point2d<T>;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::North => Point2d {
                x: self.x,
                y: self.y - T::one(),
            },
            Dir::South => Point2d {
                x: self.x,
                y: self.y + T::one(),
            },
            Dir::West => Point2d {
                x: self.x - T::one(),
                y: self.y,
            },
            Dir::East => Point2d {
                x: self.x + T::one(),
                y: self.y,
            },
        }
    }
}

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
