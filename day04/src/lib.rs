#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref INPUT: Vec<&'static str> = include_str!("../input").split("\n\n").collect();
    static ref BYR_RE: Regex = Regex::new(r"^(\d{4})$").unwrap();
    static ref IYR_RE: Regex = Regex::new(r"^(\d{4})$").unwrap();
    static ref EYR_RE: Regex = Regex::new(r"^(\d{4})$").unwrap();
    static ref HGT_RE: Regex = Regex::new(r"^(\d+)((?:in)|(?:cm))$").unwrap();
    static ref HCL_RE: Regex = Regex::new(r"^#[\da-f]{6}$").unwrap();
    static ref ECL_RE: Regex = Regex::new(r"^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$").unwrap();
    static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
}

fn solve<F: Fn(&str, Option<&str>) -> Option<bool>>(input: &[&str], check: F) -> usize {
    input
        .iter()
        .map(|line| line.split_ascii_whitespace())
        .filter_map(|pp| {
            pp.filter_map(|p| {
                let mut i = p.split(':');
                let k = i.next().unwrap();
                let v = i.next();
                match (k, check(k, v)) {
                    ("cid", _) => None,
                    (_, Some(true)) => Some((k, true)),
                    (_, Some(false)) => Some((k, false)),
                    (_, None) => unimplemented!(),
                }
            })
            .try_fold(HashSet::new(), |mut s, (k, v)| {
                if s.contains(k) || !v {
                    Err(false)
                } else {
                    Ok({
                        s.insert(k);
                        s
                    })
                }
            })
            .and_then(|s| if s.len() == 7 { Ok(1) } else { Err(false) })
            .ok()
        })
        .count()
}

fn solve_1(input: &[&str]) -> usize {
    solve(input, |_, _| Some(true))
}

fn solve_2(input: &[&str]) -> usize {
    solve(input, |k, v| match k {
        "byr" => v
            .map(|v| {
                BYR_RE
                    .captures(v)
                    .map(|cap| {
                        cap[1]
                            .parse::<u32>()
                            .map(|v| (1920..=2002).contains(&v))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            })
            .or(Some(false)),
        "iyr" => v
            .map(|v| {
                IYR_RE
                    .captures(v)
                    .map(|cap| {
                        cap[1]
                            .parse::<u32>()
                            .map(|v| (2010..=2020).contains(&v))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            })
            .or(Some(false)),
        "eyr" => v
            .map(|v| {
                EYR_RE
                    .captures(v)
                    .map(|cap| {
                        cap[1]
                            .parse::<u32>()
                            .map(|v| (2020..=2030).contains(&v))
                            .unwrap_or(false)
                    })
                    .unwrap_or(false)
            })
            .or(Some(false)),
        "hgt" => v
            .map(|v| {
                HGT_RE
                    .captures(v)
                    .map(|cap| match (cap[1].parse::<u32>(), &cap[2]) {
                        (Ok(v), "in") => (59..=76).contains(&v),
                        (Ok(v), "cm") => (150..=193).contains(&v),
                        _ => false,
                    })
                    .unwrap_or(false)
            })
            .or(Some(false)),
        "hcl" => v.map(|v| HCL_RE.is_match(v)).or(Some(false)),
        "ecl" => v.map(|v| ECL_RE.is_match(v)).or(Some(false)),
        "pid" => v.map(|v| PID_RE.is_match(v)).or(Some(false)),
        _ => None,
    })
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

    lazy_static! {
        static ref INPUT: Vec<&'static str> = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#
            .split("\n\n")
            .collect();
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(&INPUT), 2);
    }

    #[test]
    fn same_results_2_1() {
        let input: Vec<&str> = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
            .split("\n\n")
            .collect();

        assert_eq!(solve_2(&input), 0);
    }

    #[test]
    fn same_results_2_2() {
        let input: Vec<&str> = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
            .split("\n\n")
            .collect();

        assert_eq!(solve_2(&input), 4);
    }

    #[test]
    fn valid_passport() {
        let input: Vec<&str> = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f"
            .split("\n\n")
            .collect();

        assert_eq!(solve_2(&input), 1);
    }

    #[test]
    fn invalid_passport() {
        let input: Vec<&str> = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f hcl:#623a2"
            .split("\n\n")
            .collect();

        assert_eq!(solve_2(&input), 0);
    }

    #[test]
    fn invalid_passport_2() {
        let input: Vec<&str> = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f hcl:#623a2f"
            .split("\n\n")
            .collect();

        assert_eq!(solve_2(&input), 0);
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
