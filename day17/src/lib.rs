#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;

lazy_static! {
    static ref INPUT: ConwayCubes = include_str!("../input").parse().expect("invalid input");
    static ref POSITIONS_3D: Vec<Coordinate> = {
        (-1i128..=1i128)
            .map(|x| {
                (-1i128..=1i128)
                    .map(move |y| {
                        (-1i128..=1i128).filter_map(move |z| {
                            if x == 0 && y == 0 && z == 0 {
                                None
                            } else {
                                Some((x, y, z, 0))
                            }
                        })
                    })
                    .flatten()
            })
            .flatten()
            .collect()
    };
    static ref POSITIONS_4D: Vec<Coordinate> = {
        (-1i128..=1i128)
            .map(|x| {
                (-1i128..=1i128)
                    .map(move |y| {
                        (-1i128..=1i128)
                            .map(move |z| {
                                (-1i128..=1i128).filter_map(move |w| {
                                    if x == 0 && y == 0 && z == 0 && w == 0 {
                                        None
                                    } else {
                                        Some((x, y, z, w))
                                    }
                                })
                            })
                            .flatten()
                    })
                    .flatten()
            })
            .flatten()
            .collect()
    };
}

type Coordinate = (i128, i128, i128, i128);

#[derive(Debug, Clone)]
struct ConwayCubes(HashSet<Coordinate>);

impl ConwayCubes {
    fn evolve(&self, positions: &[Coordinate]) -> ConwayCubes {
        let mut set = HashSet::new();

        let mut v = HashSet::new();
        for (x, y, z, w) in self.iter() {
            let mut count = 0;
            for (dx, dy, dz, dw) in positions.iter() {
                let target = (x + dx, y + dy, z + dz, w + dw);
                count += self.get(&target).map_or(0, |_| 1);
                v.insert(target);
            }

            let p = (*x, *y, *z, *w);
            if self.contains(&p) {
                if (2..=3).contains(&count) {
                    set.insert(p);
                }
            } else if count == 3 {
                set.insert(p);
            }
        }

        for (x, y, z, w) in v {
            let mut count = 0;
            for (dx, dy, dz, dw) in positions.iter() {
                let target = (x + dx, y + dy, z + dz, w + dw);
                count += self.get(&target).map_or(0, |_| 1);
            }

            let p = (x, y, z, w);
            if self.contains(&p) {
                if (2..=3).contains(&count) {
                    set.insert(p);
                }
            } else if count == 3 {
                set.insert(p);
            }
        }

        ConwayCubes(set)
    }

    fn evolve_3d(&self) -> ConwayCubes {
        self.evolve(&POSITIONS_3D)
    }

    fn evolve_4d(&self) -> ConwayCubes {
        self.evolve(&POSITIONS_4D)
    }
}

impl std::str::FromStr for ConwayCubes {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(ConwayCubes(
            input
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.chars().enumerate().filter_map(move |(x, c)| match c {
                        '#' => Some(Ok((x as i128, y as i128, 0, 0))),
                        '.' => None,
                        _ => Some(Err(format!(
                            "invalid char at ({}, {}, {}, {}): {}",
                            x, y, 0, 0, c
                        ))),
                    })
                })
                .flatten()
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl std::ops::Deref for ConwayCubes {
    type Target = HashSet<Coordinate>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn solve_1(input: &ConwayCubes) -> usize {
    (0..6)
        .fold(input.clone(), |state, _| state.evolve_3d())
        .len()
}

fn solve_2(input: &ConwayCubes) -> usize {
    (0..6)
        .fold(input.clone(), |state, _| state.evolve_4d())
        .len()
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
        static ref INPUT: ConwayCubes = r".#.
..#
###"
        .parse()
        .expect("invalid input");
    }

    #[test]
    fn same_results_part_1() {
        assert_eq!(solve_1(&INPUT), 112);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        b.iter(part_1);
    }

    #[test]
    fn same_results_part_2() {
        assert_eq!(solve_2(&INPUT), 848);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part_2);
    }
}
