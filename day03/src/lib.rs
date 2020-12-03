#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref INPUT: TreeMap = TreeMap::new(include_str!("../input"));
}

struct TreeMap {
    map: Vec<Vec<bool>>,
    x_dim: usize,
    y_dim: usize,
}

impl TreeMap {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<bool>> = input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        let x_dim = map[0].len();
        let y_dim = map.len();

        Self { map, x_dim, y_dim }
    }

    fn walk(&self, (dx, dy): (usize, usize)) -> usize {
        let (mut x, mut y) = (0, 0);
        let mut count = 0;

        while y + dy < self.y_dim {
            x = (x + dx) % self.x_dim;
            y += dy;
            if self.map[y][x] {
                count += 1;
            }
        }

        count
    }
}

fn solve_1(tree_map: &TreeMap) -> usize {
    tree_map.walk((3, 1))
}

fn solve_2(tree_map: &TreeMap) -> usize {
    tree_map.walk((1, 1))
        * tree_map.walk((3, 1))
        * tree_map.walk((5, 1))
        * tree_map.walk((7, 1))
        * tree_map.walk((1, 2))
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
        static ref INPUT: TreeMap = TreeMap::new(
            "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
        );
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(&INPUT), 7);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&INPUT), 336);
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
