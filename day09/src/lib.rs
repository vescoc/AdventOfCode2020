#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref INPUT: Vec<u64> = parse(include_str!("../input"));
}

fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse().expect("invalid number"))
        .collect()
}

fn solve_1(input: &[u64], length: usize) -> u64 {
    input
        .windows(length + 1)
        .try_fold((), |_, seq| {
            let target = seq[seq.len() - 1];

            let mut found = false;

            'outer: for (i, a) in seq.iter().enumerate().take(seq.len() - 2) {
                for b in seq.iter().take(seq.len() - 1).skip(i) {
                    if target == a + b {
                        found = true;
                        break 'outer;
                    }
                }
            }

            if found {
                Ok(())
            } else {
                Err(target)
            }
        })
        .expect_err("not found")
}

fn solve_2(input: &[u64], length: usize) -> u64 {
    let target = solve_1(input, length);

    let sums = input
        .iter()
        .scan(0, |s, value| {
            *s += value;
            Some(*s)
        })
        .collect::<Vec<_>>();

    for (i, a) in sums.iter().enumerate().take(sums.len() - 1) {
        for (j, b) in sums.iter().enumerate().skip(i + 1) {
            if target == b - a {
                let (min, max) = input
                    .iter()
                    .take(j + 1)
                    .skip(i + 1)
                    .fold((u64::MAX, u64::MIN), |(min, max), &value| {
                        (std::cmp::min(min, value), std::cmp::max(max, value))
                    });
                return min + max;
            }
        }
    }

    unimplemented!()
}

pub fn part_1() -> u64 {
    solve_1(&INPUT, 25)
}

pub fn part_2() -> u64 {
    solve_2(&INPUT, 25)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    lazy_static! {
        static ref INPUT: Vec<u64> = parse(
            r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
        );
    }

    #[test]
    fn same_results_part_1() {
        assert_eq!(solve_1(&INPUT, 5), 127);
    }

    #[test]
    fn same_results_part_2() {
        assert_eq!(solve_2(&INPUT, 5), 62);
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
