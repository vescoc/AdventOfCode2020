#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::collections::{HashSet, VecDeque};

lazy_static! {
    static ref INPUT: Data = include_str!("../input").parse().expect("invalid input");
}

struct Data(Vec<u32>, Vec<u32>);

impl std::str::FromStr for Data {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split("\n\n");
        let mut parse = || {
            parts
                .next()
                .ok_or_else(|| "expecting player".to_string())
                .and_then(|part| {
                    part.lines()
                        .skip(1)
                        .map(|line| {
                            line.parse()
                                .map_err(|_| format!("invalid number: {}", line))
                        })
                        .collect()
                })
        };

        Ok(Data(parse()?, parse()?))
    }
}

enum Winner {
    Player1,
    Player2,
}

struct Game {
    player_1: VecDeque<u32>,
    player_2: VecDeque<u32>,
    history: HashSet<(VecDeque<u32>, VecDeque<u32>)>,
}

impl Game {
    fn new(player_1: &[u32], player_2: &[u32]) -> Self {
        Self {
            player_1: player_1.iter().copied().collect(),
            player_2: player_2.iter().copied().collect(),
            history: HashSet::new(),
        }
    }

    fn play(&mut self) -> (Winner, &mut [u32]) {
        use Winner::*;

        loop {
            {
                let k = (self.player_1.to_owned(), self.player_2.to_owned());
                if self.history.contains(&k) {
                    break (Player1, self.player_1.make_contiguous());
                } else {
                    self.history.insert(k);
                }
            }

            match (self.player_1.pop_front(), self.player_2.pop_front()) {
                (Some(card_1), Some(card_2))
                    if (card_1 as usize) <= self.player_1.len()
                        && (card_2 as usize) <= self.player_2.len() =>
                {
                    match Game::new(
                        &self.player_1.make_contiguous()[0..card_1 as usize],
                        &self.player_2.make_contiguous()[0..card_2 as usize],
                    )
                    .play()
                    {
                        (Player1, _) => {
                            self.player_1.push_back(card_1);
                            self.player_1.push_back(card_2);
                        }
                        (Player2, _) => {
                            self.player_2.push_back(card_2);
                            self.player_2.push_back(card_1);
                        }
                    }
                }
                (Some(card_1), Some(card_2)) if card_1 > card_2 => {
                    self.player_1.push_back(card_1);
                    self.player_1.push_back(card_2);
                }
                (Some(card_1), Some(card_2)) if card_1 < card_2 => {
                    self.player_2.push_back(card_2);
                    self.player_2.push_back(card_1);
                }
                (Some(card), None) => {
                    self.player_1.push_front(card);
                    break (Player1, self.player_1.make_contiguous());
                }
                (None, Some(card)) => {
                    self.player_2.push_front(card);
                    break (Player2, self.player_2.make_contiguous());
                }
                u => unreachable!(format!(
                    "c: {:?} p1: {:?} p2: {:?}",
                    u, self.player_1, self.player_2
                )),
            }
        }
    }
}

fn solve_1(input: &Data) -> usize {
    let mut player_1 = input.0.iter().copied().collect::<VecDeque<_>>();
    let mut player_2 = input.1.iter().copied().collect::<VecDeque<_>>();
    loop {
        match (player_1.pop_front(), player_2.pop_front()) {
            (Some(card_1), Some(card_2)) if card_1 > card_2 => {
                player_1.push_back(card_1);
                player_1.push_back(card_2);
            }
            (Some(card_1), Some(card_2)) if card_2 > card_1 => {
                player_2.push_back(card_2);
                player_2.push_back(card_1);
            }
            (Some(card), None) => {
                player_1.push_front(card);
                break;
            }
            (None, Some(card)) => {
                player_2.push_front(card);
                break;
            }
            _ => unreachable!(),
        }
    }

    let mut winner = if player_1.is_empty() {
        player_2
    } else {
        player_1
    };

    winner.make_contiguous().reverse();

    winner
        .iter()
        .enumerate()
        .map(|(i, v)| *v as usize * (i + 1))
        .sum()
}

fn solve_2(input: &Data) -> usize {
    let mut game = Game::new(&input.0, &input.1);

    let (_, cards) = game.play();

    cards.reverse();

    cards
        .iter()
        .enumerate()
        .map(|(i, v)| *v as usize * (i + 1))
        .sum()
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
        static ref INPUT: Data = r"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
        .parse()
        .expect("invalid input");
    }

    #[test]
    fn same_results_part_1() {
        assert_eq!(solve_1(&INPUT), 306);
    }

    #[test]
    fn same_results_part_2() {
        assert_eq!(solve_2(&INPUT), 291);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part_2);
    }
}
