#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::{collections::HashSet, hash::Hash, iter::FromIterator, std::ops::Deref};

lazy_static! {
    static ref INPUT: Vec<&'static str> = include_str!("../input").split("\n\n").collect();
}

struct SetIntersection<T>(HashSet<T>);

impl<T> Deref for SetIntersection<T> {
    type Target = HashSet<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Eq + Hash + Clone> FromIterator<HashSet<T>> for SetIntersection<T> {
    fn from_iter<I: IntoIterator<Item = HashSet<T>>>(iter: I) -> Self {
        SetIntersection(
            iter.into_iter()
                .fold(None, |s: Option<HashSet<T>>, set| {
                    s.map(|c| c.intersection(&set).cloned().collect())
                        .or(Some(set))
                })
                .unwrap_or_else(HashSet::new),
        )
    }
}

fn solve_1(input: &[&str]) -> usize {
    input
        .iter()
        .map(|group| {
            group
                .chars()
                .filter(|c| ('a'..='z').contains(c))
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

fn solve_2(input: &[&str]) -> usize {
    input
        .iter()
        .map(|group| {
            group
                .split_ascii_whitespace()
                .map(|ans| ans.chars().collect::<HashSet<_>>())
                .collect::<SetIntersection<_>>()
                .len()
        })
        .sum()
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
        static ref INPUT: Vec<&'static str> = r"abc

a
b
c

ab
ac

a
a
a
a

b"
        .split("\n\n")
        .collect();
    }

    #[test]
    fn same_results_part_1() {
        assert_eq!(solve_1(&INPUT), 11)
    }

    #[test]
    fn same_results_part_2() {
        assert_eq!(solve_2(&INPUT), 6)
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
