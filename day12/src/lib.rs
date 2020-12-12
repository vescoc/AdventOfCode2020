#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref INPUT: Vec<Move> = Move::parse(include_str!("../input")).expect("invalid input");
}

type Integer = i128;

enum Move {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    RotateLeft(i32),
    RotateRight(i32),
    Forward(i32),
}

impl Move {
    fn parse(input: &str) -> Result<Vec<Move>, String> {
        input.lines().map(|line| line.parse()).collect()
    }
}

impl std::str::FromStr for Move {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        use Move::*;

        let (m, d) = line.split_at(1);
        match (m, d.parse::<i32>()) {
            (_, Err(_)) => Err(format!("invalid number: {}", d)),
            ("N", Ok(v)) => Ok(North(v)),
            ("S", Ok(v)) => Ok(South(v)),
            ("E", Ok(v)) => Ok(East(v)),
            ("W", Ok(v)) => Ok(West(v)),
            ("L", Ok(v)) => Ok(RotateLeft(v)),
            ("R", Ok(v)) => Ok(RotateRight(v)),
            ("F", Ok(v)) => Ok(Forward(v)),
            (_, _) => Err(format!("invalid move: {}", m)),
        }
    }
}

trait Action {
    fn rotate_left(&self, v: i32) -> Self;
    fn rotate_right(&self, v: i32) -> Self;
    fn forward(&self, p: (Integer, Integer), v: i32) -> (Integer, Integer);
}

#[derive(PartialEq, Copy, Clone)]
enum Facing {
    North,
    South,
    East,
    West,
}

impl Action for Facing {
    fn rotate_left(&self, v: i32) -> Self {
        use Facing::*;

        const DIR: [Facing; 4] = [North, West, South, East];

        let (i, _) = DIR.iter().enumerate().find(|(_, v)| self == *v).unwrap();

        DIR[(i + (v as usize / 90)) % 4]
    }

    fn rotate_right(&self, v: i32) -> Self {
        use Facing::*;

        const DIR: [Facing; 4] = [North, East, South, West];

        let (i, _) = DIR.iter().enumerate().find(|(_, v)| self == *v).unwrap();

        DIR[(i + (v as usize / 90)) % 4]
    }

    fn forward(&self, (x, y): (Integer, Integer), v: i32) -> (Integer, Integer) {
        use Facing::*;
        match self {
            North => (x, y + Integer::from(v)),
            South => (x, y - Integer::from(v)),
            East => (x + Integer::from(v), y),
            West => (x - Integer::from(v), y),
        }
    }
}

struct Waypoint(Integer, Integer);

impl Waypoint {
    fn rotate(&self, v: i32) -> Self {
        let (cosv, sinv) = (
            [1, 0, -1, 0][((v / 90 + 4) % 4) as usize],
            [0, 1, 0, -1][((v / 90 + 4) % 4) as usize],
        );

        Waypoint(
            self.0 * cosv - self.1 * sinv,
            self.0 * sinv + self.1 * cosv,
        )
    }
}

impl Action for Waypoint {
    fn rotate_left(&self, v: i32) -> Self {
        self.rotate(v)
    }

    fn rotate_right(&self, v: i32) -> Self {
        self.rotate(-v)
    }

    fn forward(&self, (x, y): (Integer, Integer), v: i32) -> (Integer, Integer) {
        (x + self.0 * Integer::from(v), y + self.1 * Integer::from(v))
    }
}

fn solve_1(moves: &[Move]) -> Integer {
    let ((x, y), _) = moves.iter().fold(
        ((Integer::from(0), Integer::from(0)), Facing::East),
        |((x, y), f), m| match m {
            Move::North(v) => ((x, y + Integer::from(*v)), f),
            Move::South(v) => ((x, y - Integer::from(*v)), f),
            Move::East(v) => ((x + Integer::from(*v), y), f),
            Move::West(v) => ((x - Integer::from(*v), y), f),
            Move::RotateLeft(v) => ((x, y), f.rotate_left(*v)),
            Move::RotateRight(v) => ((x, y), f.rotate_right(*v)),
            Move::Forward(v) => (f.forward((x, y), *v), f),
        },
    );

    Integer::abs(x) + Integer::abs(y)
}

fn solve_2(moves: &[Move]) -> Integer {
    let ((x, y), _) = moves.iter().fold(
        (
            (Integer::from(0), Integer::from(0)),
            Waypoint(Integer::from(10), Integer::from(1)),
        ),
        |((x, y), Waypoint(wx, wy)), m| match m {
            Move::North(v) => ((x, y), Waypoint(wx, wy + Integer::from(*v))),
            Move::South(v) => ((x, y), Waypoint(wx, wy - Integer::from(*v))),
            Move::East(v) => ((x, y), Waypoint(wx + Integer::from(*v), wy)),
            Move::West(v) => ((x, y), Waypoint(wx - Integer::from(*v), wy)),
            Move::RotateLeft(v) => ((x, y), Waypoint(wx, wy).rotate_left(*v)),
            Move::RotateRight(v) => ((x, y), Waypoint(wx, wy).rotate_right(*v)),
            Move::Forward(v) => (Waypoint(wx, wy).forward((x, y), *v), Waypoint(wx, wy)),
        },
    );

    Integer::abs(x) + Integer::abs(y)
}

pub fn part_1() -> Integer {
    solve_1(&INPUT)
}

pub fn part_2() -> Integer {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    lazy_static! {
        static ref INPUT: Vec<Move> = Move::parse(
            r"F10
N3
F7
R90
F11"
        )
        .expect("invalid input");
    }

    #[test]
    fn same_results_part_1() {
        assert_eq!(solve_1(&INPUT), 25);
    }

    #[test]
    fn same_results_part_2() {
        assert_eq!(solve_2(&INPUT), 286);
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
