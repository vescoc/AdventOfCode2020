#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;

lazy_static! {
    static ref INPUT: Vec<Instruction> =
        Instruction::parse(include_str!("../input")).expect("invalid input");
}

#[derive(Debug)]
enum Instruction {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

impl Instruction {
    fn parse(input: &str) -> Result<Vec<Instruction>, String> {
        input.lines().map(|line| line.parse()).collect()
    }
}

impl std::str::FromStr for Instruction {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        use Instruction::*;

        let mut parts = line.split_ascii_whitespace();
        match (parts.next(), parts.next()) {
            (Some(opcode), Some(value)) => match (opcode, value.parse()) {
                ("nop", Ok(value)) => Ok(NOP(value)),
                ("acc", Ok(value)) => Ok(ACC(value)),
                ("jmp", Ok(value)) => Ok(JMP(value)),
                _ => Err(format!("invalid instruction: {}", line)),
            },
            _ => Err(format!("invalid instruction: {}", line)),
        }
    }
}

fn solve_1(input: &[Instruction]) -> i32 {
    use Instruction::*;

    let mut set = HashSet::new();
    let mut pc = 0;
    let mut acc = 0;

    while !set.contains(&pc) {
        set.insert(pc);
        match input[pc] {
            NOP(_) => pc += 1,
            ACC(value) => {
                acc += value;
                pc += 1;
            }
            JMP(value) => {
                pc = (pc as i32 + value) as usize;
            }
        }
    }

    acc
}

fn solve_2(input: &[Instruction]) -> i32 {
    use Instruction::*;

    let mut change_idx = 0;
    loop {
        let mut changed_pc = None;

        let mut count = 0;
        let mut set = HashSet::new();
        let mut pc = 0;
        let mut acc = 0;

        while !set.contains(&pc) {
            set.insert(pc);

            let instruction = match (input.get(pc), changed_pc) {
                (None, _) => return acc,
                (Some(ACC(value)), _) => ACC(*value),
                (Some(NOP(value)), None) => {
                    if count == change_idx {
                        changed_pc = Some(pc);
                        JMP(*value)
                    } else {
                        count += 1;
                        NOP(*value)
                    }
                }
                (Some(NOP(value)), Some(changed_pc)) => {
                    if pc == changed_pc {
                        JMP(*value)
                    } else {
                        NOP(*value)
                    }
                }
                (Some(JMP(value)), None) => {
                    if count == change_idx {
                        changed_pc = Some(pc);
                        NOP(*value)
                    } else {
                        count += 1;
                        JMP(*value)
                    }
                }
                (Some(JMP(value)), Some(changed_pc)) => {
                    if pc == changed_pc {
                        NOP(*value)
                    } else {
                        JMP(*value)
                    }
                }
            };

            match instruction {
                NOP(_) => pc += 1,
                ACC(value) => {
                    acc += value;
                    pc += 1;
                }
                JMP(value) => {
                    pc = (pc as i32 + value) as usize;
                }
            }
        }

        change_idx += 1;
    }
}

pub fn part_1() -> i32 {
    solve_1(&INPUT)
}

pub fn part_2() -> i32 {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    lazy_static! {
        static ref INPUT: Vec<Instruction> = Instruction::parse(
            r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
        )
        .expect("invalid input");
    }

    #[test]
    fn same_results_part_1() {
        assert_eq!(solve_1(&INPUT), 5);
    }

    #[test]
    fn same_results_part_2() {
        assert_eq!(solve_2(&INPUT), 8);
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
