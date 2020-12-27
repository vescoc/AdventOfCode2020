use std::collections::{HashMap, HashSet};

const SINS: [isize; 4] = [0, 1, 0, -1];
const CONS: [isize; 4] = [1, 0, -1, 0];
const DELTA: [[isize; 2]; 4] = [[0, 0], [-1, 0], [-1, -1], [0, -1]];

pub const TOP_INDEX: usize = 0;
pub const RIGHT_INDEX: usize = 1;
pub const BOTTOM_INDEX: usize = 2;
pub const LEFT_INDEX: usize = 3;

lazy_static! {
    static ref ROTATIONS: Vec<[[isize; 2]; 2]> = (0..4)
        .map(|i| [[CONS[i], -SINS[i]], [SINS[i], CONS[i]]])
        .collect();
}

#[derive(Clone)]
pub struct Tiles<T, O: TileOptional<T> = TileOptionalU32>(HashMap<u128, Tile<T, O>>);

impl<T, O: TileOptional<T>> std::ops::Deref for Tiles<T, O> {
    type Target = HashMap<u128, Tile<T, O>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, O: TileOptional<T>> std::str::FromStr for Tiles<T, O> {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            input
                .split("\n\n")
                .map(|tile| {
                    let mut lines = tile.split('\n');

                    lines
                        .next()
                        .ok_or_else(|| "id not found".to_string())
                        .and_then(|line| {
                            line[5..5 + 4].parse().map_err(|_| "invalid id".to_string())
                        })
                        .and_then(|id| Tile::new_from_lines(lines).map(|tile| (id, tile)))
                })
                .collect::<Result<HashMap<_, _>, _>>()?,
        ))
    }
}

impl<T: std::fmt::Debug, O: TileOptional<T>> std::fmt::Debug for Tiles<T, O> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (id, tile) in &self.0 {
            fmt.write_fmt(format_args!("Tile: {}\n", id))?;
            fmt.write_fmt(format_args!("{:?}\n", tile))?;
        }

        Ok(())
    }
}

pub trait TileOptional<T> {
    fn calc_signs(_image: &[Vec<TileCell>]) -> (HashSet<T>, Vec<(T, T)>);
}

pub struct TileOptionalU32;

impl TileOptionalU32 {
    fn calc_sign<'a, I: IntoIterator<Item = &'a TileCell>>(v: I) -> (u32, u32) {
        use TileCell::*;

        let (mut res, mut m, mut count) = (0, 1, 0);
        for c in v {
            if let On = c {
                res += m;
            }
            m <<= 1;
            count += 1;
        }

        (res, res.reverse_bits() >> (32 - count))
    }
}

pub struct TileOptionalNop;

impl TileOptional<()> for TileOptionalNop {
    fn calc_signs(_image: &[Vec<TileCell>]) -> (HashSet<()>, Vec<((), ())>) {
        (HashSet::new(), Vec::new())
    }
}

impl TileOptional<u32> for TileOptionalU32 {
    fn calc_signs(image: &[Vec<TileCell>]) -> (HashSet<u32>, Vec<(u32, u32)>) {
        let top = Self::calc_sign(&image[0]);
        let right = Self::calc_sign(image.iter().map(|row| row.last().unwrap()));
        let bottom = Self::calc_sign(image.last().unwrap().iter().rev());
        let left = Self::calc_sign(image.iter().rev().map(|row| &row[0]));

        let mut set = HashSet::new();
        let mut vec = Vec::new();
        let mut insert = |(a, b)| {
            set.insert(a);
            set.insert(b);
            vec.push((a, b));
        };

        insert(top);
        insert(right);
        insert(bottom);
        insert(left);

        (set, vec)
    }
}

pub struct Tile<T, O: TileOptional<T> = TileOptionalU32> {
    pub image: Vec<Vec<TileCell>>,
    pub edge_set: HashSet<T>,
    pub edge_vec: Vec<(T, T)>,
    _t: std::marker::PhantomData<O>,
}

impl<T: Clone, O: TileOptional<T>> Clone for Tile<T, O> {
    fn clone(&self) -> Self {
        Self {
            image: self.image.clone(),
            edge_set: self.edge_set.clone(),
            edge_vec: self.edge_vec.clone(),
            _t: self._t,
        }
    }
}

impl<T, O: TileOptional<T>> Tile<T, O> {
    fn new_from_lines<'a, I: Iterator<Item = &'a str>>(lines: I) -> Result<Self, String> {
        use TileCell::*;

        let image = lines
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Ok(Empty),
                        '#' => Ok(On),
                        _ => Err(format!("invalid cell tile: {}", c)),
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        if image.is_empty() {
            return Err("invalid tile: width zero".to_string());
        }
        if image.len() != image[0].len() {
            return Err(format!(
                "invalid tile: width {} != height {}",
                image.len(),
                image[0].len()
            ));
        }

        let (edge_set, edge_vec) = O::calc_signs(&image);

        Ok(Self {
            image,
            edge_set,
            edge_vec,
            _t: std::marker::PhantomData,
        })
    }

    pub fn new_from_image(image: Vec<Vec<TileCell>>) -> Self {
        let (edge_set, edge_vec) = O::calc_signs(&image);

        Self {
            image,
            edge_set,
            edge_vec,
            _t: std::marker::PhantomData,
        }
    }

    pub fn rotate(&mut self, angle: isize) -> &mut Self {
        let angle = ((angle + 4) % 4) as usize;
        let edge = self.image.len();
        let middle = edge as isize / 2;
        let (dx, dy) = if edge % 2 == 0 {
            (DELTA[angle][0], DELTA[angle][1])
        } else {
            (0, 0)
        };

        let matrix = ROTATIONS[angle];
        let rot = |(x, y)| {
            let (x, y) = (x as isize - middle, y as isize - middle);
            let (x, y) = (
                x * matrix[0][0] + y * matrix[0][1],
                x * matrix[1][0] + y * matrix[1][1],
            );
            ((x + middle + dx) as usize, (y + middle + dy) as usize)
        };

        let image = (0..edge)
            .map(|y| {
                (0..edge)
                    .map(|x| {
                        let (tx, ty) = rot((x, y));
                        self.image[ty][tx]
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let (edge_set, edge_vec) = O::calc_signs(&image);

        self.image = image;
        self.edge_set = edge_set;
        self.edge_vec = edge_vec;

        self
    }

    pub fn flip_h(&mut self) -> &mut Self {
        for row in self.image.iter_mut() {
            row.reverse();
        }

        let (edge_set, edge_vec) = O::calc_signs(&self.image);

        self.edge_set = edge_set;
        self.edge_vec = edge_vec;

        self
    }

    pub fn flip_v(&mut self) -> &mut Self {
        self.image.reverse();

        let (edge_set, edge_vec) = O::calc_signs(&self.image);

        self.edge_set = edge_set;
        self.edge_vec = edge_vec;

        self
    }

    #[allow(dead_code)]
    fn get_image(&self) -> String {
        use TileCell::*;

        let r = self
            .image
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| match c {
                        On => '#',
                        Empty => '.',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>();
        r.join("\n")
    }

    pub fn get_mask(&self) -> Vec<u128> {
        self.image
            .iter()
            .map(|row| {
                row.iter().fold(0, |v, c| match c {
                    TileCell::On => (v << 1) | 1,
                    TileCell::Empty => v << 1,
                })
            })
            .collect()
    }
}

impl<T: Eq + std::hash::Hash, O: TileOptional<T>> Tile<T, O> {
    pub fn find(&self, set: &HashSet<T>) -> Option<usize> {
        for (i, vec) in self.edge_vec.iter().enumerate() {
            if set.contains(&vec.0) && set.contains(&vec.1) {
                return Some(i);
            }
        }

        None
    }
}

impl<T: std::fmt::Debug, O: TileOptional<T>> std::fmt::Debug for Tile<T, O> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use TileCell::*;

        for (s, v) in vec!["top", "right", "bottom", "left"]
            .iter()
            .zip(self.edge_vec.iter())
        {
            fmt.write_fmt(format_args!("{}: {:?}\n", s, v))?;
        }

        for row in &self.image {
            for t in row {
                fmt.write_str(match t {
                    On => "#",
                    Empty => ".",
                })?;
            }
            fmt.write_str("\n")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum TileCell {
    Empty,
    On,
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref INPUT: &'static Tile<u32, TileOptionalU32> =
            crate::INPUT.values().next().unwrap();
    }

    #[test]
    fn test_rotate_3_90() {
        let mut tile: Tile<u32> = Tile::new_from_lines(
            r"#.#
.#.
..#"
            .lines(),
        )
        .expect("invalid input");

        tile.rotate(1);

        assert_eq!(
            tile.get_image(),
            r"#.#
.#.
#.."
        );
    }

    #[test]
    fn test_rotate_3_0() {
        let image = r"#..
.#.
..#";

        let mut tile: Tile<u32> = Tile::new_from_lines(image.lines()).expect("invalid input");

        tile.rotate(0);

        assert_eq!(tile.get_image(), image);
    }

    #[test]
    fn test_rotate_3_180() {
        let image = r"#..
.#.
..#";

        let mut tile: Tile<u32> = Tile::new_from_lines(image.lines()).expect("invalid input");
        tile.rotate(2);

        assert_eq!(tile.get_image(), image);
    }

    #[test]
    fn test_rotate_2_90() {
        let mut tile: Tile<u32> = Tile::new_from_lines(
            r"#.
.#"
            .lines(),
        )
        .expect("invalid input");

        tile.rotate(1);

        assert_eq!(
            tile.get_image(),
            r".#
#."
        );
    }

    #[test]
    fn test_rotate_2_0() {
        let image = r"#.
.#";

        let mut tile: Tile<u32> = Tile::new_from_lines(image.lines()).expect("invalid input");

        tile.rotate(0);

        assert_eq!(tile.get_image(), image);
    }

    #[test]
    fn test_rotate_2_180() {
        let image = r"#.
.#";

        let mut tile: Tile<u32> = Tile::new_from_lines(image.lines()).expect("invalid input");

        tile.rotate(2);

        assert_eq!(tile.get_image(), image);
    }

    #[test]
    fn test_flip_h_2() {
        let image = r"##
.#";

        let mut tile: Tile<u32> = Tile::new_from_lines(image.lines()).expect("invalid input");

        tile.flip_h();

        assert_eq!(
            tile.get_image(),
            r"##
#."
        );
    }

    #[test]
    fn test_flip_v_2() {
        let image = r"##
.#";

        let mut tile: Tile<u32> = Tile::new_from_lines(image.lines()).expect("invalid input");

        tile.flip_v();

        assert_eq!(
            tile.get_image(),
            r".#
##"
        );
    }

    #[test]
    fn test_flip_v() {
        let mut tile: Tile<u32> = INPUT.to_owned();

        tile.flip_v();

        println!("tile orig\n{:?}", *INPUT);
        println!("tile flipped\n{:?}", tile);

        assert_eq_flip(tile.edge_vec[BOTTOM_INDEX], INPUT.edge_vec[TOP_INDEX]);
        assert_eq_flip(tile.edge_vec[TOP_INDEX], INPUT.edge_vec[BOTTOM_INDEX]);
        assert_eq_flip(tile.edge_vec[LEFT_INDEX], INPUT.edge_vec[LEFT_INDEX]);
        assert_eq_flip(tile.edge_vec[RIGHT_INDEX], INPUT.edge_vec[RIGHT_INDEX]);
    }

    #[test]
    fn test_flip_h() {
        let mut tile: Tile<u32> = INPUT.to_owned();

        tile.flip_h();

        println!("tile orig\n{:?}", *INPUT);
        println!("tile flipped\n{:?}", tile);

        assert_eq_flip(tile.edge_vec[LEFT_INDEX], INPUT.edge_vec[RIGHT_INDEX]);
        assert_eq_flip(tile.edge_vec[RIGHT_INDEX], INPUT.edge_vec[LEFT_INDEX]);
        assert_eq_flip(tile.edge_vec[TOP_INDEX], INPUT.edge_vec[TOP_INDEX]);
        assert_eq_flip(tile.edge_vec[BOTTOM_INDEX], INPUT.edge_vec[BOTTOM_INDEX]);
    }

    fn assert_eq_flip<T: std::cmp::PartialEq>(ta: (T, T), tb: (T, T)) {
        assert!(ta.0 == tb.1 && ta.1 == tb.0);
    }

    #[test]
    fn test_reverse() {
        let mut v = vec![1, 2, 3, 4, 5];

        v.reverse();

        assert_eq!(v, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_vec() {
        let mut v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        v.reverse();

        assert_eq!(v, vec![vec![7, 8, 9], vec![4, 5, 6], vec![1, 2, 3]]);
    }

    #[test]
    fn test_rotate_0() {
        let mut tile: Tile<u32> = INPUT.to_owned();

        tile.rotate(0);

        println!("tile orig\n{:?}", *INPUT);
        println!("tile flipped\n{:?}", tile);

        assert_eq!(tile.edge_vec[TOP_INDEX], INPUT.edge_vec[TOP_INDEX]);
        assert_eq!(tile.edge_vec[RIGHT_INDEX], INPUT.edge_vec[RIGHT_INDEX]);
        assert_eq!(tile.edge_vec[BOTTOM_INDEX], INPUT.edge_vec[BOTTOM_INDEX]);
        assert_eq!(tile.edge_vec[LEFT_INDEX], INPUT.edge_vec[LEFT_INDEX]);
    }

    #[test]
    fn test_rotate_1() {
        let mut tile: Tile<u32> = INPUT.to_owned();

        tile.rotate(1);

        println!("tile orig\n{:?}", *INPUT);
        println!("tile flipped\n{:?}", tile);

        assert_eq!(tile.edge_vec[TOP_INDEX], INPUT.edge_vec[RIGHT_INDEX]);
        assert_eq!(tile.edge_vec[RIGHT_INDEX], INPUT.edge_vec[BOTTOM_INDEX]);
        assert_eq!(tile.edge_vec[BOTTOM_INDEX], INPUT.edge_vec[LEFT_INDEX]);
        assert_eq!(tile.edge_vec[LEFT_INDEX], INPUT.edge_vec[TOP_INDEX]);
    }

    #[test]
    fn test_rotate_2() {
        let mut tile: Tile<u32> = INPUT.to_owned();

        tile.rotate(2);

        println!("tile orig\n{:?}", *INPUT);
        println!("tile flipped\n{:?}", tile);

        assert_eq!(tile.edge_vec[TOP_INDEX], INPUT.edge_vec[BOTTOM_INDEX]);
        assert_eq!(tile.edge_vec[RIGHT_INDEX], INPUT.edge_vec[LEFT_INDEX]);
        assert_eq!(tile.edge_vec[BOTTOM_INDEX], INPUT.edge_vec[TOP_INDEX]);
        assert_eq!(tile.edge_vec[LEFT_INDEX], INPUT.edge_vec[RIGHT_INDEX]);
    }

    #[test]
    fn test_rotate_3() {
        let mut tile: Tile<u32> = INPUT.to_owned();

        tile.rotate(3);

        println!("tile orig\n{:?}", *INPUT);
        println!("tile flipped\n{:?}", tile);

        assert_eq!(tile.edge_vec[TOP_INDEX], INPUT.edge_vec[LEFT_INDEX]);
        assert_eq!(tile.edge_vec[RIGHT_INDEX], INPUT.edge_vec[TOP_INDEX]);
        assert_eq!(tile.edge_vec[BOTTOM_INDEX], INPUT.edge_vec[RIGHT_INDEX]);
        assert_eq!(tile.edge_vec[LEFT_INDEX], INPUT.edge_vec[BOTTOM_INDEX]);
    }
}
