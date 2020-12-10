#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use itertools::Itertools;
use std::collections::HashSet;

lazy_static! {
    static ref INPUT: Vec<u32> = parse(include_str!("../input"));
}

fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse().expect("invalid number"))
        .collect()
}

fn solve_1(input: &[u32]) -> [u32; 3] {
    let mut adapters = input.iter().copied().collect::<Vec<u32>>();

    adapters.sort_unstable();

    let [a, b, c] = adapters
        .windows(2)
        .map(|d| d[1] - d[0])
        .fold([0, 0, 0], |mut d, v| {
            d[v as usize - 1] += 1;
            d
        });

    [a + 1, b, c + 1]
}

fn solve_2_r(adapters: &[u32], removed: HashSet<usize>) -> usize {
    let filtered = || {
        adapters
            .iter()
            .enumerate()
            .filter_map(|(i, v)| if removed.contains(&i) { None } else { Some(v) })
    };

    if adapters.len() < 2 {
        1
    } else if !filtered().tuple_windows().all(|(a, b)| b - a <= 3) {
        0
    } else {
        let start_idx = removed.iter().max().unwrap_or(&0) + 1;
        (start_idx..adapters.len() - 1)
            .map(|i| {
                solve_2_r(
                    adapters,
                    removed.iter().cloned().chain(std::iter::once(i)).collect(),
                )
            })
            .sum::<usize>()
            + 1
    }
}

fn solve_2(input: &[u32]) -> usize {
    let mut adapters = input
        .iter()
        .copied()
        .chain(std::iter::once(0))
        .chain(std::iter::once(input.iter().max().unwrap() + 3))
        .collect::<Vec<_>>();

    adapters.sort_unstable();

    std::iter::once(0)
        .chain(adapters.windows(2).enumerate().filter_map(|(i, v)| {
            if v[1] - v[0] == 3 {
                Some(i)
            } else {
                None
            }
        }))
        .tuple_windows()
        .map(|(a, b)| solve_2_r(&adapters[a..=b], HashSet::new()))
        .product()
}

pub fn part_1() -> u32 {
    let r = solve_1(&INPUT);

    r[0] * r[2]
}

pub fn part_2() -> usize {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    lazy_static! {
        static ref INPUT_1: Vec<u32> = parse(
            r"16
10
15
5
1
11
7
19
6
12
4",
        );
        static ref INPUT_2: Vec<u32> = parse(
            r"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3",
        );
    }

    #[test]
    fn same_results_part_1_1() {
        assert_eq!(solve_1(&INPUT_1), [7, 0, 5]);
    }

    #[test]
    fn same_results_part_1_2() {
        assert_eq!(solve_1(&INPUT_2), [22, 0, 10]);
    }

    #[test]
    fn same_results_part_2_1() {
        assert_eq!(solve_2(&INPUT_1), 8);
    }

    #[test]
    fn same_results_part_2_2() {
        assert_eq!(solve_2(&INPUT_2), 19208);
    }

    #[test]
    fn test_solve_2_r_split() {
        assert_eq!(
            solve_2_r(&[1, 4], HashSet::new())
                * solve_2_r(&[4, 5, 7, 8, 9], HashSet::new()),
            5
        );
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
