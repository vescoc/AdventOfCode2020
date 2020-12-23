#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref INPUT: [u32; 9] = parse("952438716");
}

fn parse(input: &str) -> [u32; 9] {
    let mut array = [0; 9];

    input.chars().enumerate().for_each(|(i, c)| {
        array[i] = c as u32 - '0' as u32;
    });

    array
}

fn solve(labels: &[u32], len: usize, i: usize) -> (usize, Vec<usize>) {
    let mut mem = Vec::with_capacity(len + 1);
    mem.resize_with(mem.capacity(), usize::default);

    labels
        .iter()
        .map(|v| *v as usize)
        .chain(*labels.iter().max().unwrap() as usize + 1..=len)
        .zip(
            labels
                .iter()
                .map(|v| *v as usize)
                .chain(*labels.iter().max().unwrap() as usize + 1..=len)
                .skip(1),
        )
        .for_each(|(a, b)| {
            mem[a] = b;
        });

    let mut current_cup = *labels.first().unwrap() as usize;

    mem[labels
        .iter()
        .map(|v| *v as usize)
        .chain(*labels.iter().max().unwrap() as usize + 1..=len)
        .last()
        .unwrap()] = current_cup;

    for _ in 0..i {
        let a = mem[current_cup];
        let b = mem[a];
        let c = mem[b];

        mem[current_cup] = mem[c];

        let destination_cup = {
            let mut destination_cup = (current_cup + len - 1) % len;
            loop {
                if destination_cup == 0 {
                    destination_cup = len;
                }

                if destination_cup == a || destination_cup == b || destination_cup == c {
                    if destination_cup == 0 {
                        destination_cup = len;
                    } else {
                        destination_cup -= 1;
                    }
                } else {
                    break destination_cup;
                }
            }
        };

        let end = mem[destination_cup];
        mem[destination_cup] = a;
        mem[c] = end;

        current_cup = mem[current_cup];
    }

    (current_cup, mem)
}

fn solve_1(labels: &[u32; 9]) -> String {
    let (_, mem) = solve(&labels[..], 9, 100);

    std::iter::successors(Some(mem[1]), |v| {
        if mem[*v] == 1 {
            None
        } else {
            Some(mem[*v])
        }
    })
    .map(|v| std::char::from_digit(v as u32, 10).unwrap())
    .collect::<String>()
}

fn solve_2(labels: &[u32; 9]) -> u64 {
    let (_, mem) = solve(&labels[..], 1_000_000, 10_000_000);

    mem[1] as u64 * mem[mem[1]] as u64
}

pub fn part_1() -> String {
    solve_1(&INPUT)
}

pub fn part_2() -> u64 {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    lazy_static! {
        static ref EXAMPLE_1: [u32; 9] = parse("389125467");
    }

    #[test]
    fn same_results_example_1() {
        assert_eq!(solve_1(&EXAMPLE_1), "67384529");
    }

    #[test]
    fn same_results_example_2_1() {
        let (_, mem) = solve(&EXAMPLE_1[..], 1_000_000, 10_000_000);

        assert_eq!(mem[1], 934001);
        assert_eq!(mem[mem[1]], 159792);
    }

    #[test]
    fn same_results_example_2() {
        assert_eq!(solve_2(&EXAMPLE_1), 149245887792);
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
