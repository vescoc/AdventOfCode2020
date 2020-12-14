#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref INPUT: Vec<Instruction> = parse(include_str!("../input")).expect("invalid input");
    static ref MASK_RE: Regex = Regex::new(r"mask = ([X01]{36})").unwrap();
    static ref MEM_RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
}

fn parse(input: &str) -> Result<Vec<Instruction>, String> {
    input.lines().map(|line| line.parse()).collect()
}

enum Instruction {
    Mask(u64, u64, u64),
    Mem(u64, u64),
}

impl std::str::FromStr for Instruction {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Instruction::*;

        if let Some(cap) = MASK_RE.captures(input) {
            let (mask_and, mask_or, mask_floating) = cap[1].chars().fold(
                (0x0, 0x0, 0x0),
                |(mask_and, mask_or, mask_floating), c| match c {
                    'X' => (
                        (mask_and << 1) | 0x1,
                        mask_or << 1,
                        mask_floating << 1 | 0x1,
                    ),
                    '1' => (
                        (mask_and << 1) | 0x1,
                        (mask_or << 1) | 0x1,
                        mask_floating << 1,
                    ),
                    '0' => (mask_and << 1, mask_or << 1, mask_floating << 1),
                    _ => unreachable!(),
                },
            );
            Ok(Mask(mask_and, mask_or, mask_floating))
        } else if let Some(cap) = MEM_RE.captures(input) {
            Ok(Mem(
                cap[1]
                    .parse()
                    .map_err(|_| "invalid mem address".to_string())?,
                cap[2].parse().map_err(|_| "invalid number".to_string())?,
            ))
        } else {
            Err(format!("invalid instruction: {}", input))
        }
    }
}

fn solve_1(instructions: &[Instruction]) -> u64 {
    use Instruction::*;

    let (memory, _) = instructions.iter().fold(
        (HashMap::new(), None),
        |(mut memory, mask), instruction| match (instruction, mask) {
            (Mask(mask_and, mask_or, mask_floating), _) => {
                (memory, Some(Mask(*mask_and, *mask_or, *mask_floating)))
            }
            (Mem(address, value), Some(Mask(mask_and, mask_or, mask_floating))) => {
                memory.insert(address, value & mask_and | mask_or);
                (memory, Some(Mask(mask_and, mask_or, mask_floating)))
            }
            _ => unreachable!("invalid state, mask undefined"),
        },
    );

    memory.values().sum()
}

fn solve_2(instructions: &[Instruction]) -> u64 {
    use Instruction::*;

    let (memory, _) = instructions.iter().fold(
        (HashMap::new(), None),
        |(mut memory, mask), instruction| match (instruction, mask) {
            (Mask(mask_and, mask_or, mask_floating), _) => {
                (memory, Some(Mask(*mask_and, *mask_or, *mask_floating)))
            }
            (
                Mem(current_address, current_value),
                Some(Mask(current_mask_and, current_mask_or, current_mask_floating)),
            ) => {
                let current_address = current_address & !current_mask_and | current_mask_or;
                let c = current_mask_floating.count_ones();
                for i in 0..(1 << c) {
                    let (mut a, mut j, mut k) = (0, 1, 1);
                    while k < (1 << c) {
                        if current_mask_floating & j != 0 {
                            if i & k != 0 {
                                a |= j;
                            }
                            k <<= 1;
                        }
                        j <<= 1;
                    }

                    memory.insert(current_address | a, *current_value);
                }

                (
                    memory,
                    Some(Mask(
                        current_mask_and,
                        current_mask_or,
                        current_mask_floating,
                    )),
                )
            }
            _ => unreachable!("invalid state, mask undefined"),
        },
    );

    memory.values().sum()
}

pub fn part_1() -> u64 {
    solve_1(&INPUT)
}

pub fn part_2() -> u64 {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn same_results_part_1() {
        let input = parse(
            r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0",
        )
        .expect("invalid input");

        assert_eq!(solve_1(&input), 165);
    }

    #[test]
    fn same_results_part_2() {
        let input = parse(
            r"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1",
        )
        .expect("invalid input");

        assert_eq!(solve_2(&input), 208);
    }

    #[test]
    fn same_address_no_mask() {
        let input = parse(
            r"mask = 000000000000000000000000000000000000
mem[42] = 100
mem[42] = 1",
        )
        .expect("invalid input");

        assert_eq!(solve_2(&input), 1);
    }

    #[test]
    fn same_address_same_mask() {
        let input = parse(
            r"mask = 000000000000000000000000X00000000000
mem[42] = 100
mem[42] = 1",
        )
        .expect("invalid input");

        assert_eq!(solve_2(&input), 2);
    }

    #[test]
    fn same_address_over_mask() {
        let input = parse(
            r"mask = 00000000000000000000000XX00000000000
mem[42] = 100
mask = 00000000000000000000000X000000000000
mem[42] = 1",
        )
        .expect("invalid input");

        assert_eq!(solve_2(&input), 202);
    }

    #[test]
    fn same_address_under_mask() {
        let input = parse(
            r"mask = 00000000000000000000000X000000000000
mem[42] = 100
mask = 00000000000000000000000XX00000000000
mem[42] = 1",
        )
        .expect("invalid input");

        assert_eq!(solve_2(&input), 4);
    }

    #[test]
    fn diff_address_same_mask() {
        let input = parse(
            r"mask = 000000000000000000000000000000000000
mem[1] = 100
mem[2] = 1",
        )
        .expect("invalid input");

        assert_eq!(solve_2(&input), 101);
    }

    #[test]
    fn diff_address_diff_mask() {
        let input = parse(
            r"mask = 0000000000000000000000000000000000X0
mem[1] = 100
mask = 000000000000000000000000000000000000
mem[2] = 1",
        )
        .expect("invalid input");

        assert_eq!(solve_2(&input), 201);
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
