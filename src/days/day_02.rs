use clap::Parser;
use pomsky_macro::pomsky;
use regex::Regex;
use std::{fs::File, io, path::PathBuf};

use crate::{commands::Runnable, days::Day};

#[derive(Parser)]
pub struct Command {
    #[clap(value_name = "FILE")]
    file: PathBuf,
    #[clap(value_enum, default_value_t=Part::Part1)]
    part: Part,
}

#[derive(clap::ValueEnum, Clone, Copy)]
pub enum Part {
    Part1,
    Part2,
}

impl Runnable for Command {
    fn run(&self) {
        let file = File::open(&self.file).expect("Impossible d'ouvrir le fichier");
        let result = Day02::new(self.part).process(io::BufReader::new(file));
        println!("The result is \"{}\".", result);
    }
}

struct Day02 {
    re_game: Regex,
    re_draw: Regex,
    part: Part,
}

impl Day02 {
    fn new(part: Part) -> Self {
        Self {
            re_game: Regex::new(pomsky! {
                let rounds = [' ' ",;" ascii_digit "red" "green" "blue"]+;
                ^ "Game " :id([digit]+) ": " :rounds(rounds) $
            })
            .unwrap(),
            re_draw: Regex::new(pomsky! {
                let color = "red" | "green" | "blue";
                ^ :count([ascii_digit]+) ' ' :color(color) $
            })
            .unwrap(),
            part,
        }
    }
}

impl Day<usize> for Day02 {
    fn process(&self, input: impl io::BufRead) -> usize {
        let common = input.lines().filter_map(|line| {
            parse_game(&line.unwrap_or("".into()), &self.re_game, &self.re_draw)
        });

        match self.part {
            Part::Part1 => common
                .filter(|game| game.is_valid_part1())
                .map(|game| game.id)
                .sum(),
            Part::Part2 => common
                .map(|game| max_cubes(&game.rounds))
                .map(|Round { red, green, blue }| red * green * blue)
                .sum(),
        }
    }
}

#[derive(Debug, PartialEq, Default)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl Game {
    fn is_valid_part1(&self) -> bool {
        self.rounds.iter().all(|round| round.is_valid_part1())
    }
}

#[derive(Debug, PartialEq, Default)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

impl Round {
    fn is_valid_part1(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

fn max_cubes(vec: &[Round]) -> Round {
    vec.iter().fold(Round::default(), |mut acc, round| {
        acc.red = acc.red.max(round.red);
        acc.green = acc.green.max(round.green);
        acc.blue = acc.blue.max(round.blue);
        acc
    })
}

fn parse_game(line: &str, re_game: &Regex, re_draw: &Regex) -> Option<Game> {
    let mut game = Game::default();
    let Some((Some(id), Some(rounds))) = re_game
        .captures(line)
        .and_then(|cap| (cap.name("id"), cap.name("rounds")).into())
    else {
        return None;
    };

    game.id = id.as_str().parse().ok()?;
    let rounds = rounds
        .as_str()
        .split("; ")
        .map(|round| parse_round(round, re_draw));

    for round in rounds {
        game.rounds.push(round?);
    }

    Some(game)
}

fn parse_round(round: &str, re_draw: &Regex) -> Option<Round> {
    let mut result = Round::default();
    for draw in round.split(", ") {
        let cap = re_draw.captures(draw)?;
        let color = cap.name("color")?.as_str();
        let count: usize = cap.name("count")?.as_str().parse().ok()?;
        match (color, count) {
            ("red", n) => result.red = n,
            ("green", n) => result.green = n,
            ("blue", n) => result.blue = n,
            _ => return None,
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regexes_compile() {
        Day02::new(Part::Part1);
    }

    #[test]
    fn regex_game_parse() {
        let day = Day02::new(Part::Part1);
        let hay = "Game 33: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let Some(groups) = day.re_game.captures(hay) else {
            panic!("No game ID found");
        };
        assert_eq!("33", &groups["id"]);
        assert_eq!(
            "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            &groups["rounds"]
        );
    }

    #[test]
    fn regex_draw_parse() {
        let day = Day02::new(Part::Part1);
        let hay = "3 blue";
        let Some(groups) = day.re_draw.captures(hay) else {
            panic!("Can't parse count and color");
        };
        assert_eq!("3", &groups["count"]);
        assert_eq!("blue", &groups["color"]);
    }

    #[test]
    fn parse_game_simple() {
        let day = Day02::new(Part::Part1);
        assert_eq!(
            parse_game(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                &day.re_game,
                &day.re_draw
            ),
            Some(Game {
                id: 1,
                rounds: vec![
                    Round {
                        blue: 3,
                        red: 4,
                        green: 0
                    },
                    Round {
                        red: 1,
                        green: 2,
                        blue: 6
                    },
                    Round {
                        green: 2,
                        red: 0,
                        blue: 0
                    }
                ]
            })
        );
    }

    #[test]
    fn parse_round_simple() {
        let day = Day02::new(Part::Part1);
        assert_eq!(
            parse_round("1 red, 1 green, 1 blue", &day.re_draw),
            Some(Round {
                red: 1,
                green: 1,
                blue: 1,
            })
        );
    }

    #[test]
    fn round_is_valid_part1() {
        let rounds_good = vec![
            Round {
                blue: 3,
                red: 12,
                green: 0,
            },
            Round {
                red: 1,
                green: 13,
                blue: 6,
            },
            Round {
                green: 2,
                red: 0,
                blue: 14,
            },
        ];

        for round in rounds_good {
            assert!(round.is_valid_part1());
        }
    }

    #[test]
    fn round_is_invalid_part1() {
        let rounds_bad = vec![
            Round {
                blue: 3,
                red: 13,
                green: 0,
            },
            Round {
                red: 1,
                green: 14,
                blue: 6,
            },
            Round {
                green: 2,
                red: 0,
                blue: 15,
            },
        ];

        for round in rounds_bad {
            assert!(!round.is_valid_part1());
        }
    }

    #[test]
    fn part2_works() {
        let file = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .as_bytes();
        let reader = io::BufReader::new(file);
        assert_eq!(Day02::new(Part::Part2).process(reader), 2286);
    }
}
