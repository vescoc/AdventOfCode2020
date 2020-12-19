#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref INPUT: (Rules, Vec<&'static str>) = parse(include_str!("../input"));
}

#[derive(Clone)]
enum Rule {
    Simple(char),
    Seq(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
}

impl std::str::FromStr for Rule {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use Rule::*;

        let parse_seq = |part: &str| {
            part.split(' ')
                .map(|v| {
                    v.parse()
                        .map_err(|_| format!("invalid number: {} input: {}", v, input))
                })
                .collect::<Result<_, _>>()
        };

        let mut parts = input.split(" | ");

        match (parts.next(), parts.next()) {
            (Some(part), None) => {
                if part.starts_with('"') {
                    Ok(Simple(
                        part[1..2]
                            .chars()
                            .next()
                            .ok_or_else(|| format!("expecting char: {}", part))?,
                    ))
                } else {
                    Ok(Seq(parse_seq(part)?))
                }
            }
            (Some(part_left), Some(part_right)) => {
                Ok(Or(parse_seq(part_left)?, parse_seq(part_right)?))
            }
            _ => Err("invalid rule format".to_string()),
        }
    }
}

#[derive(Clone)]
struct Rules(HashMap<usize, Rule>);

impl Rules {
    fn is_match<'a>(&self, input: &'a str, rule_index: usize) -> Vec<(&'a str, &'a str)> {
        use Rule::*;

        let match_seq = |seq: &[usize]| {
            seq.iter()
                .try_fold(vec![(0, input)], |v, rule_index| {
                    let v = v
                        .iter()
                        .map(|(i, input)| {
                            self.is_match(input, *rule_index)
                                .iter()
                                .map(|(p, r)| (i + p.len(), *r))
                                .collect::<Vec<_>>()
                        })
                        .flatten()
                        .collect::<Vec<_>>();
                    if v.is_empty() {
                        Err(())
                    } else {
                        Ok(v)
                    }
                })
                .map_or_else(
                    |_| Vec::new(),
                    |v| v.iter().map(|(i, r)| (&input[0..*i], *r)).collect(),
                )
        };

        match self.0.get(&rule_index) {
            Some(Simple(c)) if !input.is_empty() => match input[0..1].chars().next() {
                Some(t) if t == *c => vec![(&input[0..1], &input[1..])],
                _ => Vec::new(),
            },
            Some(Seq(seq)) => match_seq(seq),
            Some(Or(left, right)) => {
                let mut v = match_seq(left);
                v.append(&mut match_seq(right));
                v
            }
            _ => Vec::new(),
        }
    }

    fn replace(&mut self, rule_index: usize, rule: Rule) {
        self.0.insert(rule_index, rule);
    }
}

impl std::str::FromStr for Rules {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rules = input
            .lines()
            .map(|line| {
                let mut parts = line.split(": ");
                match (parts.next(), parts.next()) {
                    (Some(index), Some(rule)) => Ok((
                        index
                            .parse()
                            .map_err(|_| format!("invalid number: {} line: {}", index, line))?,
                        rule.parse()?,
                    )),
                    _ => Err(format!("invalid rule entry: {}", line)),
                }
            })
            .collect::<Result<_, _>>()?;

        Ok(Self(rules))
    }
}

fn parse(input: &str) -> (Rules, Vec<&str>) {
    let mut parts = input.split("\n\n");

    match (parts.next(), parts.next()) {
        (Some(rules), Some(messages)) => (
            rules.parse().expect("invalid rules"),
            messages.lines().collect(),
        ),
        _ => panic!("invalid format"),
    }
}

fn solve_1(rules: &Rules, messages: &[&str]) -> usize {
    messages
        .iter()
        .filter_map(|line| {
            if rules.is_match(line, 0).iter().any(|(_, r)| r.is_empty()) {
                Some(())
            } else {
                None
            }
        })
        .count()
}

fn solve_2(rules: &Rules, messages: &[&str]) -> usize {
    let mut rules = rules.clone();

    rules.replace(8, Rule::Or(vec![42], vec![42, 8]));
    rules.replace(11, Rule::Or(vec![42, 31], vec![42, 11, 31]));

    messages
        .iter()
        .filter_map(|line| {
            if rules.is_match(line, 0).iter().any(|(_, r)| r.is_empty()) {
                Some(())
            } else {
                None
            }
        })
        .count()
}

pub fn part_1() -> usize {
    solve_1(&INPUT.0, &INPUT.1)
}

pub fn part_2() -> usize {
    solve_2(&INPUT.0, &INPUT.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    lazy_static! {
        static ref EXAMPLE_1: (Rules, Vec<&'static str>) = parse(
            r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#
        );
        static ref EXAMPLE_2: (Rules, Vec<&'static str>) = parse(
            r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#
        );
    }

    #[test]
    fn same_results_part_1_1() {
        assert_eq!(solve_1(&EXAMPLE_1.0, &EXAMPLE_1.1), 2);
    }

    #[test]
    fn same_results_part_2_1() {
        assert_eq!(solve_1(&EXAMPLE_2.0, &EXAMPLE_2.1), 3);
    }

    #[test]
    fn same_results_part_2_2() {
        assert_eq!(solve_2(&EXAMPLE_2.0, &EXAMPLE_2.1), 12);
    }

    #[test]
    fn test_is_match_simple() {
        assert_eq!(EXAMPLE_1.0.is_match("ab", 4), vec![("a", "b")]);
        assert_eq!(EXAMPLE_1.0.is_match("ba", 4), Vec::new());
    }

    #[test]
    fn test_is_match_seq() {
        let rules = r#"0: 1 2
1: "a"
2: "b""#
            .parse::<Rules>()
            .expect("invalid rules");

        assert_eq!(rules.is_match("ab", 0), vec![("ab", "")]);
    }

    #[test]
    fn test_is_match_or() {
        let rules = r#"0: 1 2 | 2 1
1: "a"
2: "b""#
            .parse::<Rules>()
            .expect("invalid rules");

        assert_eq!(rules.is_match("ab", 0), vec![("ab", "")]);
        assert_eq!(rules.is_match("ba", 0), vec![("ba", "")]);
    }

    #[test]
    #[ignore]
    fn test_is_match_rec() {
        let rules = r#"0: 1 2 | 0
1: "a"
2: "b""#
            .parse::<Rules>()
            .expect("invalid rules");

        assert_eq!(rules.is_match("ab", 0), vec![("ab", "")]);
        assert_eq!(rules.is_match("abab", 0), vec![("ab", "ab"), ("abab", "")]);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }

    #[bench]
    #[ignore]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part_2);
    }
}
