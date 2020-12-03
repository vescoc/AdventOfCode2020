#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

mod bit_set;

const RANGE: usize = 2020;

lazy_static! {
    static ref INPUT: Vec<usize> = include_str!("../input")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
}

fn solve_1(input: &[usize]) -> usize {
    let mut set = bit_set::BitSet::new(RANGE);

    for &i in input {
        let d = RANGE - i;
        if set.get(d) {
            return d * i;
        }
        set.set(i, true);
    }

    panic!("not found");
}

fn solve_2(input: &[usize]) -> usize {
    for i in 0..input.len() - 2 {
        for j in i + 1..input.len() - 1 {
            let s = input[i] + input[j];
            if s < RANGE {
                let d = RANGE - s;
                for t in j + 1..input.len() {
                    if input[t] == d {
                        return input[i] * input[j] * input[t];
                    }
                }
            }
        }
    }

    panic!("not found")
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

    static INPUT: [usize; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(&INPUT), 514579);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&INPUT), 241861950);
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
