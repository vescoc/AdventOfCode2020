#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;

lazy_static! {
    static ref INPUT: Vec<&'static str> = include_str!("../input").lines().collect();
    static ref POSITIONS: Vec<Cube> = vec![
        Cube(1, 0, -1),
        Cube(1, -1, 0),
        Cube(0, -1, 1),
        Cube(-1, 0, 1),
        Cube(-1, 1, 0),
        Cube(0, 1, -1)
    ];
}

type Coordinate = i32;

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Cube(Coordinate, Coordinate, Coordinate);

impl<T> std::ops::Add<T> for Cube
where
    T: std::borrow::Borrow<Cube>,
{
    type Output = Cube;

    fn add(self, b: T) -> Self::Output {
        let Cube(xa, ya, za) = self;
        let Cube(xb, yb, zb) = b.borrow();

        Self(xa + xb, ya + yb, za + zb)
    }
}

impl std::iter::Sum for Cube {
    fn sum<I: Iterator<Item = Self>>(i: I) -> Self {
        i.fold(Cube(0, 0, 0), std::ops::Add::add)
    }
}

trait Walk {
    fn walk(&self) -> WalkIterator;
}

impl Walk for &str {
    fn walk(&self) -> WalkIterator {
        WalkIterator { input: self }
    }
}

struct WalkIterator<'a> {
    input: &'a str,
}

impl<'a> Iterator for WalkIterator<'a> {
    type Item = Cube;

    fn next(&mut self) -> Option<Self::Item> {
        match self.input {
            v if v.starts_with("ne") => {
                self.input = &self.input[2..];
                Some(Cube(1, 0, -1))
            }
            v if v.starts_with('e') => {
                self.input = &self.input[1..];
                Some(Cube(1, -1, 0))
            }
            v if v.starts_with("se") => {
                self.input = &self.input[2..];
                Some(Cube(0, -1, 1))
            }
            v if v.starts_with("sw") => {
                self.input = &self.input[2..];
                Some(Cube(-1, 0, 1))
            }
            v if v.starts_with('w') => {
                self.input = &self.input[1..];
                Some(Cube(-1, 1, 0))
            }
            v if v.starts_with("nw") => {
                self.input = &self.input[2..];
                Some(Cube(0, 1, -1))
            }
            _ => None,
        }
    }
}

fn solve_1(input: &[&str]) -> usize {
    input
        .iter()
        .map(|line| line.walk().sum::<Cube>())
        .fold(HashSet::new(), |mut set, c| {
            if set.contains(&c) {
                set.remove(&c);
            } else {
                set.insert(c);
            }
            set
        })
        .len()
}

fn solve_2(input: &[&str]) -> usize {
    let mut tiles =
        input
            .iter()
            .map(|line| line.walk().sum::<Cube>())
            .fold(HashSet::new(), |mut set, c| {
                if set.contains(&c) {
                    set.remove(&c);
                } else {
                    set.insert(c);
                }
                set
            });

    for _i in 0..100 {
        let (mut new_tiles, mut tiles_to_check) = (HashSet::new(), HashSet::new());
        for &c in tiles.iter() {
            let mut count = 0;
            for dc in POSITIONS.iter() {
                let t = c + dc;
                if tiles.contains(&t) {
                    count += 1;
                } else {
                    tiles_to_check.insert(t.to_owned());
                }
            }

            if (1..=2).contains(&count) {
                new_tiles.insert(c);
            }
        }

        for c in tiles_to_check {
            let mut count = 0;
            for dc in POSITIONS.iter() {
                let t = c + dc;
                if tiles.contains(&t) {
                    count += 1;
                }
            }

            if count == 2 {
                new_tiles.insert(c);
            }
        }

        tiles = new_tiles;
    }

    tiles.len()
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
        static ref INPUT: Vec<&'static str> = r"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"
            .lines()
            .collect();
    }

    #[test]
    fn flip_reference() {
        assert_eq!("nwwswee".walk().sum::<Cube>(), Cube(0, 0, 0));
    }

    #[test]
    fn same_results_example_1() {
        assert_eq!(solve_1(&INPUT), 10);
    }

    #[test]
    fn same_results_example_2() {
        assert_eq!(solve_2(&INPUT), 2208);
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
