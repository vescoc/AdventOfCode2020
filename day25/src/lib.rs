#![feature(test)]
extern crate test;

lazy_static::lazy_static! {
    static ref INPUT: Vec<u64> = include_str!("../input")
        .lines()
        .map(|line| line
             .parse()
             .expect("invalid number"))
        .collect();
}

const SUBJECT_NUMBER: u64 = 7;
const MODULUS: u64 = 20201227;

fn loop_size(pbk: u64) -> usize {
    std::iter::successors(Some(1), |v| Some((v * SUBJECT_NUMBER) % MODULUS))
        .enumerate()
        .take_while(|(_, v)| *v != pbk)
        .last()
        .map(|(i, _)| i)
        .unwrap()
        + 1
}

fn transform(subject_number: u64, loop_size: usize) -> u64 {
    std::iter::successors(Some(1), |v| Some((v * subject_number) % MODULUS))
        .nth(loop_size)
        .unwrap()
}

fn solve_1(pbk1: u64, pbk2: u64) -> u64 {
    transform(pbk1, loop_size(pbk2))
}

pub fn part_1() -> u64 {
    solve_1(INPUT[0], INPUT[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn loop_size_5764801_is_8() {
        assert_eq!(loop_size(5764801), 8);
    }

    #[test]
    fn loop_size_17807724_is_11() {
        assert_eq!(loop_size(17807724), 11);
    }

    #[test]
    fn transform_17807724_is_14897079() {
        assert_eq!(transform(17807724, 8), 14897079);
    }

    #[test]
    fn transform_5764801_is_14897079() {
        assert_eq!(transform(5764801, 11), 14897079);
    }

    #[test]
    fn same_results_example_1() {
        assert_eq!(solve_1(17807724, 5764801), 14897079);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }
}
