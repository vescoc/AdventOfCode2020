#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref INPUT: Game = "2,15,0,9,1,20".parse().expect("invalid input");
}

struct Game {
    input: Vec<u32>,
}

impl std::str::FromStr for Game {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input
            .split(',')
            .map(|value| {
                value
                    .parse()
                    .map_err(|_| format!("invalid number: {}", value))
            })
            .collect::<Result<_, _>>()?;

        Ok(Game { input })
    }
}

impl Game {
    fn iter(&self) -> std::iter::Chain<std::iter::Copied<std::slice::Iter<'_, u32>>, GameIterator> {
        let rounds = self
            .input
            .iter()
            .enumerate()
            .map(|(i, v)| (*v, i + 1))
            .collect::<HashMap<_, _>>();

        let last = *self.input.last().unwrap();
        let round = self.input.len();
        let first = true;

        self.input.iter().copied().chain(GameIterator {
            rounds,
            last,
            round,
            first,
        })
    }
}

struct GameIterator {
    rounds: HashMap<u32, usize>,
    last: u32,
    round: usize,
    first: bool,
}

impl Iterator for GameIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.last = if self.first {
            self.rounds.insert(self.last, self.round);
            0
        } else {
            let last_round = self.rounds.get(&self.last).copied().unwrap();
            self.rounds.insert(self.last, self.round);
            (self.round - last_round) as u32
        };

        self.first = !self.rounds.contains_key(&self.last);
        self.round += 1;

        Some(self.last)
    }
}

fn solve_1(game: &Game) -> u32 {
    game.iter().nth(2020 - 1).unwrap()
}

fn solve_2(game: &Game) -> u32 {
    game.iter().nth(30_000_000 - 1).unwrap()
}

pub fn part_1() -> u32 {
    solve_1(&INPUT)
}

pub fn part_2() -> u32 {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn same_results_part_1() {
        assert_eq!(solve_1(&"0,3,6".parse().expect("invalid input")), 436);
    }

    #[test]
    fn same_results_part_1_1() {
        assert_eq!(solve_1(&"1,3,2".parse().expect("invalid input")), 1);
    }

    #[test]
    fn same_results_part_1_2() {
        assert_eq!(solve_1(&"2,1,3".parse().expect("invalid input")), 10);
    }

    #[test]
    fn same_results_part_1_3() {
        assert_eq!(solve_1(&"1,2,3".parse().expect("invalid input")), 27);
    }

    #[test]
    fn same_results_part_1_4() {
        assert_eq!(solve_1(&"2,3,1".parse().expect("invalid input")), 78);
    }

    #[test]
    fn same_results_part_1_5() {
        assert_eq!(solve_1(&"3,2,1".parse().expect("invalid input")), 438);
    }

    #[test]
    fn same_results_part_1_6() {
        assert_eq!(solve_1(&"3,1,2".parse().expect("invalid input")), 1836);
    }

    #[test]
    #[ignore]
    fn same_results_part_2() {
        assert_eq!(solve_2(&"0,3,6".parse().expect("invalid input")), 175594);
    }

    #[test]
    #[ignore]
    fn same_results_part_2_1() {
        assert_eq!(solve_2(&"1,3,2".parse().expect("invalid input")), 2578);
    }

    #[test]
    #[ignore]
    fn same_results_part_2_2() {
        assert_eq!(solve_2(&"2,1,3".parse().expect("invalid input")), 3544142);
    }

    #[test]
    #[ignore]
    fn same_results_part_2_3() {
        assert_eq!(solve_2(&"1,2,3".parse().expect("invalid input")), 261214);
    }

    #[test]
    #[ignore]
    fn same_results_part_2_4() {
        assert_eq!(solve_2(&"2,3,1".parse().expect("invalid input")), 6895259);
    }

    #[test]
    #[ignore]
    fn same_results_part_2_5() {
        assert_eq!(solve_2(&"3,2,1".parse().expect("invalid input")), 18);
    }

    #[test]
    #[ignore]
    fn same_results_part_2_6() {
        assert_eq!(solve_2(&"3,1,2".parse().expect("invalid input")), 362);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }

    #[bench]
    #[ignore]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part_2);
    }
}
