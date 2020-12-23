#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref INPUT: [u32; 9] = parse("952438716");
}

trait CrapCups<T> {
    fn crap_cups(&self) -> CrapCupsIter<T>;
}

struct CrapCupsIter<T> {
    labels: [T; 9],
    current_cup_index: usize,
}

impl CrapCups<u32> for [u32; 9] {
    fn crap_cups(&self) -> CrapCupsIter<u32> {
        CrapCupsIter {
            labels: self.to_owned(),
            current_cup_index: 0,
        }
    }
}

impl Iterator for CrapCupsIter<u32> {
    type Item = CapCupsValue;

    fn next(&mut self) -> Option<CapCupsValue> {
        let current = self.labels.to_owned();

        let find = |data: [_; 6], value| {
            data.iter()
                .enumerate()
                .find_map(|(i, v)| if *v == value { Some(i) } else { None })
                .unwrap()
        };

        let current_cup = current[self.current_cup_index];

        let destination_cup = {
            let mut destination_cup = current_cup - 1;
            loop {
                if destination_cup == 0 {
                    destination_cup = 9;
                }

                if (1..4).any(|i| current[(self.current_cup_index + i) % 9] == destination_cup) {
                    destination_cup = (destination_cup + 9 - 1) % 9;
                } else {
                    break destination_cup;
                }
            }
        };

        let mut tmp = [0; 9 - 3];
        let (mut d, mut c) = (0, 0);
        for i in 0..9 {
            if (1..4).any(|s| (self.current_cup_index + s) % 9 == i) {
                c += 1;
            } else {
                tmp[d] = current[c];
                d += 1;
                c += 1;
            }
        }

        let destination_cup_index = find(tmp, destination_cup);
        let current_cup_index = find(tmp, current_cup);

        let (mut d, mut s) = (self.current_cup_index, current_cup_index);
        for _ in 0..6 {
            if s == destination_cup_index {
                self.labels[d] = tmp[s];
                self.labels[(d + 1) % 9] = current[(self.current_cup_index + 1) % 9];
                self.labels[(d + 2) % 9] = current[(self.current_cup_index + 2) % 9];
                self.labels[(d + 3) % 9] = current[(self.current_cup_index + 3) % 9];

                d = (d + 4) % 9;
                s = (s + 1) % 6;
            } else {
                self.labels[d] = tmp[s];

                d = (d + 1) % 9;
                s = (s + 1) % 6;
            }
        }

        self.current_cup_index = (self.current_cup_index + 1) % 9;

        Some(CapCupsValue(current))
    }
}

struct CapCupsValue([u32; 9]);

impl std::fmt::Debug for CapCupsValue {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("{:?}: {}", self.0, &self.to_string()))
    }
}

impl ToString for CapCupsValue {
    fn to_string(&self) -> String {
        let index = self
            .0
            .iter()
            .enumerate()
            .find_map(|(i, v)| if *v == 1 { Some(i) } else { None })
            .unwrap();
        (1..9)
            .map(|i| (self.0[(i + index) % 9] + '0' as u32) as u8 as char)
            .collect()
    }
}

fn parse(input: &str) -> [u32; 9] {
    let mut array = [0; 9];

    input.chars().enumerate().for_each(|(i, c)| {
        array[i] = c as u32 - '0' as u32;
    });

    array
}

fn solve_1(labels: &[u32; 9]) -> String {
    labels.crap_cups().nth(100).unwrap().to_string()
}

fn solve_2(_labels: &[u32; 9]) -> String {
    todo!()
}

pub fn part_1() -> String {
    solve_1(&INPUT)
}

pub fn part_2() -> String {
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
    fn same_results_example_1_1() {
        let value = EXAMPLE_1.crap_cups().next().unwrap();

        assert_eq!(value.to_string(), "25467389");
    }

    #[test]
    fn same_results_example_1_2() {
        let value = EXAMPLE_1.crap_cups().nth(1).unwrap();

        assert_eq!(value.to_string(), "54673289");
    }

    #[test]
    fn same_results_example_1_3() {
        let value = EXAMPLE_1.crap_cups().nth(2).unwrap();

        assert_eq!(value.to_string(), "32546789");
    }

    #[test]
    fn same_results_example_1_4() {
        let value = EXAMPLE_1.crap_cups().nth(3).unwrap();

        assert_eq!(value.to_string(), "34672589");
    }

    #[test]
    fn same_results_example_1_5() {
        let value = EXAMPLE_1.crap_cups().nth(4).unwrap();

        assert_eq!(value.to_string(), "32584679");
    }

    #[test]
    fn same_results_example_1_10() {
        assert_eq!(
            EXAMPLE_1.crap_cups().nth(10).unwrap().to_string(),
            "92658374"
        );
    }

    #[test]
    fn same_results_example_1_100() {
        assert_eq!(solve_1(&EXAMPLE_1), "67384529");
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
