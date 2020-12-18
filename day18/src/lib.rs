#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref INPUT: &'static str = include_str!("../input");
}

fn parse_expr<I>(expr: &mut I) -> Result<u64, String>
where
    I: Iterator<Item = char>,
{
    let mut current = None;
    let mut current_op = None;
    while let Some(value) = expr.next() {
        match (value, current, current_op) {
            (' ', _, _) => {}
            ('0'..='9', None, None) => current = Some(value as u64 - '0' as u64),
            ('0'..='9', Some(c), Some('+')) => {
                current = Some(c + value as u64 - '0' as u64);
                current_op = None;
            }
            ('0'..='9', Some(c), Some('*')) => {
                current = Some(c * (value as u64 - '0' as u64));
                current_op = None;
            }
            ('(', None, None) => current = Some(parse_expr(expr)?),
            ('(', Some(c), Some('+')) => {
                current = Some(c + parse_expr(expr)?);
                current_op = None;
            }
            ('(', Some(c), Some('*')) => {
                current = Some(c * parse_expr(expr)?);
                current_op = None;
            }
            (')', None, None) => return Err("empty sub expr".to_string()),
            (')', Some(c), None) => return Ok(c),
            (op, Some(_), None) if op == '+' || op == '*' => current_op = Some(value),
            _ => {
                return Err(format!(
                    "invalid state: ({}, {:?}, {:?})",
                    value, current, current_op
                ))
            }
        }
    }

    current.ok_or_else(|| "empty expression".to_string())
}

fn parse_expr_p<I>(expr: &mut I) -> Result<u64, String>
where
    I: Iterator<Item = char>,
{
    let mut current = Vec::new();
    let mut current_op = Vec::new();
    while let Some(value) = expr.next() {
        match (value, current.last().copied(), current_op.last().copied()) {
            (' ', _, _) => {}
            ('0'..='9', None, None) => {
                current.push(value as u64 - '0' as u64);
            }
            ('0'..='9', Some(c), Some('+')) => {
                current.pop();
                current_op.pop();

                current.push(c + value as u64 - '0' as u64);
            }
            ('0'..='9', Some(_), Some('*')) => {
                current.push(value as u64 - '0' as u64);
            }
            ('(', None, None) => {
                current.push(parse_expr_p(expr)?);
            }
            ('(', Some(c), Some('+')) => {
                current.pop();
                current_op.pop();

                current.push(c + parse_expr_p(expr)?);
            }
            ('(', Some(_), Some('*')) => {
                current.push(parse_expr_p(expr)?);
            }
            (')', None, None) => return Err("empty sub expr".to_string()),
            (')', Some(_), None) => break,
            (op, Some(_), _) if op == '+' || op == '*' => {
                current_op.push(op);
            }
            _ => break,
        }
    }

    Ok(current.iter().product())
}

fn parse(expr: &str) -> Result<u64, String> {
    parse_expr(&mut expr.chars())
}

fn parse_p(expr: &str) -> Result<u64, String> {
    parse_expr_p(&mut expr.chars())
}

fn solve_1(input: &str) -> u64 {
    input
        .lines()
        .map(|exp| parse(exp).expect("invalid expr"))
        .sum()
}

fn solve_2(input: &str) -> u64 {
    input
        .lines()
        .map(|exp| parse_p(exp).expect("invalid expr"))
        .sum()
}

pub fn part_1() -> u64 {
    solve_1(&INPUT)
}

pub fn part_2() -> u64 {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn same_results_part_1_1() {
        assert_eq!(solve_1("1 + 2 * 3 + 4 * 5 + 6"), 71);
    }

    #[test]
    fn same_results_part_1_2() {
        assert_eq!(solve_1("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    }

    #[test]
    fn same_results_part_1_3() {
        assert_eq!(solve_1("2 * 3 + (4 * 5)"), 26);
    }

    #[test]
    fn same_results_part_1_4() {
        assert_eq!(solve_1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    }

    #[test]
    fn same_results_part_1_5() {
        assert_eq!(solve_1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    }

    #[test]
    fn same_results_part_1_6() {
        assert_eq!(
            solve_1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn same_results_part_2_1() {
        assert_eq!(solve_2("1 + 2 * 3 + 4 * 5 + 6"), 231);
    }

    #[test]
    fn same_results_part_2_2() {
        assert_eq!(solve_2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    }

    #[test]
    fn same_results_part_2_3() {
        assert_eq!(solve_2("2 * 3 + (4 * 5)"), 46);
    }

    #[test]
    fn same_results_part_2_4() {
        assert_eq!(solve_2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    }

    #[test]
    fn same_results_part_2_5() {
        assert_eq!(solve_2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
    }

    #[test]
    fn same_results_part_2_6() {
        assert_eq!(
            solve_2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }

    #[test]
    fn same_results_part_2_sum() {
        assert_eq!(solve_2("1 + 2"), 3);
    }

    #[test]
    fn same_results_part_2_mul() {
        assert_eq!(solve_2("1 * 2"), 2);
    }

    #[test]
    fn same_results_part_2_sum_1() {
        assert_eq!(solve_2("1 + (2 * 3)"), 7);
    }

    #[test]
    fn same_results_part_2_mul_1() {
        assert_eq!(solve_2("1 * (2 + 3)"), 5);
    }

    #[test]
    fn same_results_part_2_sum_2() {
        assert_eq!(solve_2("1 + 2 * 3 + 4"), 21);
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
