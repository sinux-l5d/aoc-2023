use clap::Parser;
use std::{fs::File, io, iter, path::PathBuf};

use crate::{commands::Runnable, days::Day};

#[derive(Parser)]
pub struct Command {
    #[clap(value_name = "FILE")]
    file: PathBuf,
}

impl Runnable for Command {
    fn run(&self) {
        let file = File::open(&self.file).expect("Impossible d'ouvrir le fichier");
        let result = Day01.process(io::BufReader::new(file));
        println!("The result is \"{}\".", result);
    }
}

struct Day01;

impl Day<usize> for Day01 {
    fn process(&self, input: impl io::BufRead) -> usize {
        input
            .lines()
            .map(|line| nb_from_line(line.unwrap_or("".into()).as_str()))
            .sum()
    }
}

fn nb_from_line(l: &str) -> usize {
    let mut it = iter_numbers(l);

    if let Some(first) = it.next() {
        let last = it.last().unwrap_or(first);
        (10 * first + last).into()
    } else {
        0
    }
}

fn iter_numbers(l: &str) -> impl Iterator<Item = u8> + '_ {
    let mut current = "".to_owned();

    l.bytes().filter_map(move |byte| match byte {
        b'0'..=b'9' => {
            current.clear();
            Some(byte - b'0')
        }
        b'a'..=b'z' => {
            current.push(byte as char);
            match str_to_nb(&current) {
                Some(SpelledNumber::Incomplete) => None,
                Some(SpelledNumber::Is(n)) => {
                    current.clear();
                    Some(n)
                }
                None => {
                    if let Some(start) = find_partial_number_end(&current) {
                        current = start.to_owned();
                    } else {
                        current.clear();
                    }
                    None
                }
            }
        }
        _ => {
            current.clear();
            None
        }
    })
}

fn find_partial_number_end(s: &str) -> Option<&str> {
    let it = iter::successors(Some(s), |prev| {
        if prev.is_empty() {
            None
        } else {
            Some(&prev[1..])
        }
    });
    for substr in it {
        if let Some(SpelledNumber::Incomplete) = str_to_nb(substr) {
            return Some(substr);
        }
    }
    None
}

#[derive(PartialEq, Debug)]
enum SpelledNumber {
    Is(u8),
    Incomplete,
}

fn str_to_nb(s: &str) -> Option<SpelledNumber> {
    const NUMBERS: &[&str; 9] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let matching = NUMBERS.iter().enumerate().find_map(|(idx, spelled)| {
        spelled
            .strip_prefix(s)
            .map(|remainder| (idx + 1, remainder))
    });
    match matching {
        Some((n, "")) => Some(SpelledNumber::Is(n as u8)),
        Some((..)) => Some(SpelledNumber::Incomplete),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let file = "1abc2\n\
                    pqr3stu8vwx\n\
                    a1b2c3d4e5f\n\
                    treb7uchet"
            .as_bytes();
        let reader = io::BufReader::new(file);
        assert_eq!(Day01.process(reader), 142);
    }

    #[test]
    fn part2_works() {
        let file = "two1nine\n\
                        eightwothree\n\
                        abcone2threexyz\n\
                        xtwone3four\n\
                        4nineeightseven2\n\
                        zoneight234\n\
                        7pqrstsixteen"
            .as_bytes();
        let reader = io::BufReader::new(file);
        assert_eq!(Day01.process(reader), 281);
    }

    #[test]
    fn parse_3_digits() {
        let s = "a1b2c3d4e5f";
        let it = iter_numbers(s);
        assert_eq!(it.collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
        assert_eq!(nb_from_line(s), 15);
    }

    #[test]
    fn parse_2_digits() {
        assert_eq!(nb_from_line("1abc2"), 12);
    }

    #[test]
    fn parse_1_digit() {
        assert_eq!(nb_from_line("1abc"), 11);
    }

    #[test]
    fn parse_empty() {
        assert_eq!(nb_from_line(""), 0);
    }

    #[test]
    fn parse_spelled_digit() {
        assert_eq!(nb_from_line("two1nine"), 29);
    }

    #[test]
    fn parse_spelled_dont_consume_unrelated() {
        assert_eq!(find_partial_number_end("oni"), Some("ni"));
        assert_eq!(iter_numbers("eightwonine").collect::<Vec<_>>(), vec![8, 9]);
        // prevent               -----x~~xxx
        // where - is Some(SpelledNumber::Is), x is None, and ~ is Some(SpelledNumber::Incomplete)
    }

    #[test]
    fn parse_mixed_digit() {
        assert_eq!(nb_from_line("4nineeightseven2"), 42)
    }

    #[test]
    fn convert_valid_spelled() {
        assert_eq!(str_to_nb("eight"), Some(SpelledNumber::Is(8)));
    }

    #[test]
    fn convert_partial_spelled() {
        assert_eq!(str_to_nb("fi"), Some(SpelledNumber::Incomplete));
    }

    #[test]
    fn convert_invalid_spelled() {
        assert_eq!(str_to_nb("z"), None);
    }
}
