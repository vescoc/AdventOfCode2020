#![feature(test)]
#![feature(destructuring_assignment)]
extern crate test;

#[macro_use]
extern crate lazy_static;

#[derive(PartialEq)]
enum InfoPart {
    Value(i128),
    X,
}

impl std::str::FromStr for InfoPart {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input == "x" {
            Ok(InfoPart::X)
        } else {
            Ok(InfoPart::Value(
                input
                    .parse()
                    .map_err(|_| format!("invalid id: {}", input))?,
            ))
        }
    }
}

struct Info {
    timestamp: Option<i128>,
    ids: Vec<InfoPart>,
}

impl std::str::FromStr for Info {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines().collect::<Vec<_>>();

        let timestamp = if lines.len() > 1 {
            Some(
                lines[0]
                    .parse()
                    .map_err(|_| format!("invalid timestamp: {}", lines[0]))?,
            )
        } else {
            None
        };

        let ids = lines
            .last()
            .ok_or_else(|| "invalid input".to_string())
            .and_then(|v| v.split(',').map(|v| v.parse::<InfoPart>()).collect())?;

        Ok(Info { timestamp, ids })
    }
}

lazy_static! {
    static ref INPUT: Info = include_str!("../input").parse().expect("invalid input");
}

#[allow(clippy::many_single_char_names)]
fn gcdex(a: i128, b: i128) -> (i128, (i128, i128)) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }

    (old_r, (old_s, old_t))
}

fn solve_1(timestamp: i128, ids: &[InfoPart]) -> i128 {
    let (id, time) = ids
        .iter()
        .filter_map(|id| {
            if let InfoPart::Value(id) = id {
                Some(id)
            } else {
                None
            }
        })
        .map(|id| (id, id - timestamp % id))
        .min_by_key(|(_, v)| *v)
        .unwrap();

    id * time
}

fn solve_2(ids: &[InfoPart]) -> i128 {
    let exprs: Vec<(i128, i128)> = ids
        .iter()
        .enumerate()
        .filter_map(|(i, id)| {
            if let InfoPart::Value(id) = id {
                Some((i as i128, *id))
            } else {
                None
            }
        })
        .collect();

    let n = exprs.iter().map(|(_, v)| v).product::<i128>();

    let mut v = exprs
        .iter()
        .map(|(i, v)| {
            let ni = n / v;
            let (_, (mi, _)) = gcdex(ni, *v);
            -i * mi * ni % n
        })
        .sum::<i128>();

    while v < 0 {
        v += n;
    }

    v % n
}

pub fn part_1() -> i128 {
    solve_1(INPUT.timestamp.expect("no timestamp"), &INPUT.ids)
}

pub fn part_2() -> i128 {
    solve_2(&INPUT.ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    lazy_static! {
        static ref INPUT: Info = r"939
7,13,x,x,59,x,31,19"
            .parse()
            .expect("invalid input");
    }

    #[test]
    fn same_results_part_1() {
        assert_eq!(
            solve_1(INPUT.timestamp.expect("no timestamp"), &INPUT.ids),
            295
        );
    }

    #[test]
    fn same_results_part_2() {
        assert_eq!(solve_2(&INPUT.ids), 1068781);
    }

    #[test]
    fn same_results_part_2_1() {
        let info = "17,x,13,19".parse::<Info>().unwrap();

        assert_eq!(solve_2(&info.ids), 3417);
    }

    #[test]
    fn same_results_part_2_2() {
        let info = "67,7,59,61".parse::<Info>().unwrap();

        assert_eq!(solve_2(&info.ids), 754018);
    }

    #[test]
    fn same_results_part_2_3() {
        let info = "67,x,7,59,61".parse::<Info>().unwrap();

        assert_eq!(solve_2(&info.ids), 779210);
    }

    #[test]
    fn same_results_part_2_4() {
        let info = "67,7,x,59,61".parse::<Info>().unwrap();

        assert_eq!(solve_2(&info.ids), 1261476);
    }

    #[test]
    fn same_results_part_2_5() {
        let info = "1789,37,47,1889".parse::<Info>().unwrap();

        assert_eq!(solve_2(&info.ids), 1202161486);
    }

    #[test]
    fn test_gcdex_1() {
        assert_eq!(gcdex(13, 19), (1, (3, -2)));
    }

    #[test]
    fn test_gcdex_2() {
        assert_eq!(gcdex(937, 397), (1, (-186, 439)));
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
