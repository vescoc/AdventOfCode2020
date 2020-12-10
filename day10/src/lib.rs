#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

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

fn solve_2(_input: &[u32]) -> u32 {
    todo!()
}

pub fn part_1() -> u32 {
    let r = solve_1(&INPUT);

    r[0] * r[2]
}

pub fn part_2() -> u32 {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn same_results_part_1_1() {
        let input = parse(
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

        assert_eq!(solve_1(&input), [7, 0, 5]);
    }

    #[test]
    fn same_results_part_1_2() {
        let input = parse(
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

        assert_eq!(solve_1(&input), [22, 0, 10]);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }
}
