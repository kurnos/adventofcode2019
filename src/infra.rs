use clipboard_win::Clipboard;
use std::fs;
use std::str;
use std::time::Instant;

pub trait InputProvider<T> {
    fn get_contentsut(self) -> T;
}

pub trait Problem<I1, I2, R1, R2> {
    fn day() -> u8;
    fn first(contents: I1) -> R1;
    fn second(contents: I2) -> R2;
}

pub struct FromFile(pub &'static str);

impl InputProvider<String> for FromFile {
    fn get_contentsut(self) -> String {
        fs::read_to_string(format!("resources/{}", self.0)).unwrap()
    }
}

pub struct Literal<T>(pub T);

impl<T> InputProvider<T> for Literal<T> {
    fn get_contentsut(self) -> T {
        self.0
    }
}

pub struct FromClipboard;

impl InputProvider<String> for FromClipboard {
    fn get_contentsut(self) -> String {
        Clipboard::new().unwrap().get_string().unwrap()
    }
}

pub fn run_day<P, I1, I2, R1, R2, IP1, IP2>(
    day: Option<u8>,
    _: P,
    first: (IP1, R1),
    second: (IP2, R2),
) where
    P: Problem<I1, I2, R1, R2>,
    R1: std::fmt::Debug + PartialEq,
    R2: std::fmt::Debug + PartialEq,
    IP1: InputProvider<I1>,
    IP2: InputProvider<I2>,
{
    let my_day = P::day();
    if day.map(|d| d == my_day).unwrap_or(true) {
        let i1 = first.0.get_contentsut();
        let t = Instant::now();
        let r1 = P::first(i1);
        let t1 = t.elapsed();
        assert_eq!(r1, first.1);

        let i2 = second.0.get_contentsut();
        let t = Instant::now();
        let r2 = P::second(i2);
        let t2 = t.elapsed();
        assert_eq!(r2, second.1);
        println!("Day{}a: {:?}", my_day, r1);
        println!("Day{}a: {:?}", my_day, r2);
        println!("{:?} ({:?} and {:?})", t1 + t2, t1, t2);
    }
}
