use std::fmt::Display;
use std::io::BufRead;

pub trait Day<T: Display> {
    fn process(&self, input: impl BufRead) -> T;
}

pub mod day_01;
pub mod day_02;
