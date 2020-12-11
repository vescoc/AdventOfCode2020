#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref INPUT: SeatingSystem = include_str!("../input").parse().expect("invalid input");
}

#[derive(PartialEq, Clone, Copy)]
enum PositionType {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

#[derive(PartialEq, Clone)]
struct SeatingSystem {
    width: usize,
    height: usize,
    layout: Vec<Vec<PositionType>>,
}

impl SeatingSystem {
    fn count_occupied_seats(&self) -> usize {
        self.layout
            .iter()
            .map(|v| {
                v.iter()
                    .filter(|&&c| c == PositionType::OccupiedSeat)
                    .count()
            })
            .sum()
    }

    fn evolve_near(&mut self) {
        use PositionType::*;

        const DELTAS: [(isize, isize); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        self.layout = self
            .layout
            .iter()
            .enumerate()
            .map(|(y, v)| {
                v.iter()
                    .enumerate()
                    .map(|(x, c)| {
                        let mut count_mem = None;
                        let mut count = || -> usize {
                            if let Some(count) = count_mem {
                                count
                            } else {
                                let c = DELTAS
                                    .iter()
                                    .filter_map(|(dx, dy)| {
                                        match self.get((x as isize + dx, y as isize + dy)) {
                                            Some(OccupiedSeat) => Some(()),
                                            _ => None,
                                        }
                                    })
                                    .count();
                                
                                count_mem = Some(c);
                                
                                c
                            }
                        };

                        match c {
                            EmptySeat if count() == 0 => OccupiedSeat,
                            OccupiedSeat if count() >= 4 => EmptySeat,
                            _ => *c,
                        }
                    })
                    .collect()
            })
            .collect();
    }

    fn evolve_range(&mut self) {
        use PositionType::*;

        const DIRECTIONS: [(isize, isize); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        self.layout = self
            .layout
            .iter()
            .enumerate()
            .map(|(y, v)| {
                v.iter()
                    .enumerate()
                    .map(|(x, c)| {
                        let mut count_mem = None;
                        let mut count = || -> usize {
                            if let Some(count) = count_mem {
                                count
                            } else {
                                let c = DIRECTIONS
                                    .iter()
                                    .map(|(dx, dy)| {
                                        (1..)
                                            .try_fold((), |_, d| {
                                                match self
                                                    .get((x as isize + dx * d, y as isize + dy * d))
                                                {
                                                    Some(OccupiedSeat) => Err(1),
                                                    Some(EmptySeat) | None => Err(0),
                                                    _ => Ok(()),
                                                }
                                            })
                                            .expect_err("invalid")
                                    })
                                    .sum();

                                count_mem = Some(c);

                                c
                            }
                        };

                        match c {
                            EmptySeat if count() == 0 => OccupiedSeat,
                            OccupiedSeat if count() >= 5 => EmptySeat,
                            _ => *c,
                        }
                    })
                    .collect()
            })
            .collect();
    }

    #[inline]
    fn get(&self, (x, y): (isize, isize)) -> Option<PositionType> {
        if x < 0 || y < 0 {
            None
        } else {
            self.layout
                .get(y as usize)
                .and_then(|v| v.get(x as usize).cloned())
        }
    }
}

impl std::str::FromStr for SeatingSystem {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        use PositionType::*;

        let layout: Vec<Vec<PositionType>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Ok(Floor),
                        '#' => Ok(OccupiedSeat),
                        'L' => Ok(EmptySeat),
                        _ => Err(format!("invalid position: {}", c)),
                    })
                    .collect::<Result<_, Self::Err>>()
            })
            .collect::<Result<_, Self::Err>>()?;

        let width = layout
            .get(0)
            .ok_or_else(|| "invalid layout: no rows".to_string())?
            .len();
        let height = layout.len();

        Ok(Self {
            layout,
            width,
            height,
        })
    }
}

fn solve<F: Fn(&mut SeatingSystem)>(seating_system: &SeatingSystem, f: F) -> usize {
    let mut current_seating_system = seating_system.clone();
    loop {
        let mut new_seating_system = current_seating_system.clone();

        f(&mut new_seating_system);

        if new_seating_system == current_seating_system {
            return new_seating_system.count_occupied_seats();
        }

        current_seating_system = new_seating_system;
    }
}

fn solve_1(seating_system: &SeatingSystem) -> usize {
    solve(seating_system, SeatingSystem::evolve_near)
}

fn solve_2(seating_system: &SeatingSystem) -> usize {
    solve(seating_system, SeatingSystem::evolve_range)
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
        static ref INPUT: SeatingSystem = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            .parse()
            .expect("invalid input");
    }

    #[test]
    fn same_results_part_1() {
        assert_eq!(solve_1(&INPUT), 37);
    }

    #[test]
    fn same_results_part_2() {
        assert_eq!(solve_2(&INPUT), 26);
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
