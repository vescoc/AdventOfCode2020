#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;

lazy_static! {
    static ref INPUT: Vec<&'static str> =
        include_str!("../input").split_ascii_whitespace().collect();
}

fn calc(data: &str) -> usize {
    let (_, _, row, _, _, col) = data.chars().fold(
        (0, 127, 0, 0, 7, 0),
        |(start_row, end_row, row, start_col, end_col, col), c| {
            let middle_row = || (end_row + start_row) / 2;
            let middle_col = || (end_col + start_col) / 2;

            match c {
                'F' => (start_row, middle_row(), start_row, start_col, end_col, col),
                'B' => (middle_row() + 1, end_row, end_row, start_col, end_col, col),
                'R' => (start_row, end_row, row, middle_col() + 1, end_col, end_col),
                'L' => (start_row, end_row, row, start_col, middle_col(), start_col),
                _ => unimplemented!(),
            }
        },
    );
    row * 8 + col
}

fn solve_1(input: &[&str]) -> usize {
    input.iter().map(|line| calc(line)).max().unwrap()
}

fn solve_2(input: &[&str]) -> usize {
    let seats = input.iter().map(|line| calc(line)).collect::<HashSet<_>>();

    let (min, max) = (*seats.iter().min().unwrap(), *seats.iter().max().unwrap());

    let (start, mut distance) = (127 / 2 * 8 + 8 / 2, 0);

    while start + distance < max || start - distance > min {
        let current_up = start + distance;
        if !seats.contains(&current_up)
            && seats.contains(&(current_up + 1))
            && seats.contains(&(current_up - 1))
        {
            return current_up;
        }

        let current_down = start - distance;
        if !seats.contains(&current_down)
            && seats.contains(&(current_down + 1))
            && seats.contains(&(current_down - 1))
        {
            return current_down;
        }

        distance += 1;
    }

    unimplemented!()
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

    #[test]
    fn same_results_part_1() {
        assert_eq!(calc("FBFBBFFRLR"), 357);
        assert_eq!(calc("BFFFBBFRRR"), 567);
        assert_eq!(calc("FFFBBBFRRR"), 119);
        assert_eq!(calc("BBFFBBFRLL"), 820);
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
