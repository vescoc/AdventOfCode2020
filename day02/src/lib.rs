#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::{num::ParseIntError, str::FromStr};

struct Password {
    p1: usize,
    p2: usize,
    c: char,
    password: String,
}

impl Password {
    fn is_valid_1(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.c).count();
        count >= self.p1 && count <= self.p2
    }

    fn is_valid_2(&self) -> bool {
        self.password
            .char_indices()
            .filter(|&(i, _)| i == self.p1 - 1 || i == self.p2 - 1)
            .filter(|&(_, c)| c == self.c)
            .count()
            == 1
    }
}

#[derive(Debug)]
struct ParseError {
    error: &'static str,
}

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        Self {
            error: "invalid number",
        }
    }
}

impl FromStr for Password {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        match RE.captures(line) {
            Some(cap) => Ok(Self {
                p1: cap[1].parse()?,
                p2: cap[2].parse()?,
                c: cap[3].chars().next().ok_or(ParseError {
                    error: "expecting a letter",
                })?,
                password: cap[4].to_string(),
            }),
            _ => Err(ParseError {
                error: "invalid format",
            }),
        }
    }
}

lazy_static! {
    static ref INPUT: Vec<&'static str> = include_str!("../input").lines().collect();
    static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();
}

fn solve<P: FnMut(&Password) -> bool>(input: &[&str], test: P) -> usize {
    input
        .iter()
        .map(|line| line.parse().expect("invalid input"))
        .filter(test)
        .count()
}

fn solve_1(input: &[&str]) -> usize {
    solve(input, Password::is_valid_1)
}

fn solve_2(input: &[&str]) -> usize {
    solve(input, Password::is_valid_2)
}

pub fn part_1() -> usize {
    solve_1(&INPUT)
}

pub fn part_2() -> usize {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    lazy_static! {
        static ref INPUT: Vec<&'static str> =
            vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc",];
        static ref INPUT_PARSED: Vec<Password> = super::INPUT
            .iter()
            .map(|line| line.parse().expect("invalid input"))
            .collect();
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(&INPUT), 2);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&INPUT), 1);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part_2);
    }

    #[bench]
    fn bench_sample_1_bis(b: &mut Bencher) {
        b.iter(|| {
            INPUT_PARSED
                .iter()
                .filter(|password| password.is_valid_1())
                .count();
        });
    }

    #[bench]
    fn bench_sample_2_bis(b: &mut Bencher) {
        b.iter(|| {
            INPUT_PARSED
                .iter()
                .filter(|password| password.is_valid_2())
                .count();
        });
    }
}
