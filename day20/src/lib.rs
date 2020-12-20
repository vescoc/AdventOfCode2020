#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet, VecDeque};

lazy_static! {
    static ref INPUT: Tiles = include_str!("../input")
        .trim()
        .parse()
        .expect("invalid input");
}

#[derive(Clone)]
struct Tiles(HashMap<u128, Tile>);

impl std::ops::Deref for Tiles {
    type Target = HashMap<u128, Tile>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::str::FromStr for Tiles {
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
                        .and_then(|id| Tile::new(lines).map(|tile| (id, tile)))
                })
                .collect::<Result<HashMap<_, _>, _>>()?,
        ))
    }
}

impl std::fmt::Debug for Tiles {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (id, tile) in &self.0 {
            fmt.write_fmt(format_args!("Tile: {}\n", id))?;
            fmt.write_fmt(format_args!("{:?}\n", tile))?;
        }

        Ok(())
    }
}

#[derive(Clone)]
struct Tile {
    #[allow(dead_code)]
    image: Vec<Vec<TileCell>>,
    top: (u32, u32),
    right: (u32, u32),
    bottom: (u32, u32),
    left: (u32, u32),
    set: HashSet<u32>,
}

impl Tile {
    fn new(lines: std::str::Split<char>) -> Result<Self, String> {
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

        let top = Tile::calc_sign(&image[0]);
        let right = Tile::calc_sign(image.iter().map(|row| row.last().unwrap()));
        let left = Tile::calc_sign(image.iter().map(|row| &row[0]));
        let bottom = Tile::calc_sign(image.last().unwrap());

        let mut set = HashSet::new();
        let mut insert = |(a, b)| {
            set.insert(a);
            set.insert(b);
        };

        insert(top);
        insert(right);
        insert(left);
        insert(bottom);

        Ok(Self {
            image,
            top,
            right,
            bottom,
            left,
            set,
        })
    }

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

impl std::fmt::Debug for Tile {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("top: {:?}\n", self.top))?;
        fmt.write_fmt(format_args!("right: {:?}\n", self.right))?;
        fmt.write_fmt(format_args!("bottom: {:?}\n", self.bottom))?;
        fmt.write_fmt(format_args!("left: {:?}\n", self.left))?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
enum TileCell {
    Empty,
    On,
}

struct Combination2<T> {
    data: Vec<T>,
    i: usize,
    j: usize,
}

impl<T: Sized + Clone> Iterator for Combination2<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.data.len();
        if len < 2 || self.i > len - 2 {
            None
        } else {
            self.j += 1;
            if self.j >= len {
                self.i += 1;
                self.j = self.i + 1;
            }
            if self.i > len - 2 {
                None
            } else {
                Some((self.data[self.i].clone(), self.data[self.j].clone()))
            }
        }
    }
}

trait Combination2B: IntoIterator
where
    Self: Sized,
    Self::Item: Clone,
{
    fn combination2(self) -> Combination2<Self::Item> {
        let data: Vec<Self::Item> = self.into_iter().collect();

        Combination2 { data, i: 0, j: 0 }
    }
}

impl<T> Combination2B for T
where
    T: IntoIterator,
    T::Item: Clone,
{
}

fn solve_1(input: &Tiles) -> u128 {
    input
        .iter()
        .combination2()
        .filter_map(|((a_id, a_tile), (b_id, b_tile))| {
            let i = a_tile.set.intersection(&b_tile.set).collect::<HashSet<_>>();
            if i.is_empty() {
                None
            } else {
                Some((a_id, b_id))
            }
        })
        .fold(HashMap::new(), |mut map, (a_id, b_id)| {
            *map.entry(a_id).or_insert(0) += 1;
            *map.entry(b_id).or_insert(0) += 1;
            map
        })
        .iter()
        .filter_map(|(id, count)| if *count == 2 { Some(*id) } else { None })
        .product()
}

fn solve_2(input: &Tiles) -> usize {
    let size = input.len();
    let edge = (size as f32).sqrt() as i128;

    let mut map: Vec<Option<u128>> = vec![None; size];

    let get = |map: &Vec<Option<u128>>, (x, y)| {
        if x >= 0 && x < edge && y >= 0 && y < edge {
            map[(x + y * edge) as usize]
        } else {
            None
        }
    };

    {
        // populate map
        let neighbors = input
            .iter()
            .combination2()
            .filter_map(|((a_id, a_tile), (b_id, b_tile))| {
                let i = a_tile.set.intersection(&b_tile.set).collect::<HashSet<_>>();
                if i.is_empty() {
                    None
                } else {
                    Some((a_id, b_id))
                }
            })
            .fold(HashMap::new(), |mut map, (a_id, b_id)| {
                map.entry(a_id).or_insert_with(HashSet::new).insert(b_id);
                map.entry(b_id).or_insert_with(HashSet::new).insert(a_id);
                map
            });

        let seed = neighbors
            .iter()
            .filter_map(|(id, v)| if v.len() == 2 { Some(id) } else { None })
            .next()
            .unwrap();

        map[0] = Some(**seed);

        let set = |map: &mut Vec<Option<u128>>, (x, y): (i128, i128), v| {
            map[(x + y * edge) as usize] = Some(v);
        };

        let mut queue = VecDeque::new();
        queue.push_back((0, 1));
        queue.push_back((1, 0));

        let mut found = HashSet::new();
        found.insert(*seed);

        while let Some((x, y)) = queue.pop_front() {
            if get(&map, (x, y)).is_some() {
                continue;
            }

            let n = match (get(&map, (x, y - 1)), get(&map, (x - 1, y))) {
                (Some(a_id), Some(b_id)) => neighbors[&a_id]
                    .intersection(&neighbors[&b_id])
                    .copied()
                    .collect(),
                (None, Some(id)) => neighbors[&id].clone(),
                (Some(id), None) => neighbors[&id].clone(),
                (None, None) => unreachable!(),
            }
            .difference(&found)
            .copied()
            .collect::<HashSet<_>>();

            let current = *n.iter().next().unwrap();
            found.insert(current);
            set(&mut map, (x, y), *current);

            if x + 1 < edge {
                queue.push_back((x + 1, y));
            }
            if y + 1 < edge {
                queue.push_back((x, y + 1));
            }
        }

        assert_eq!(found.len(), input.len());
    }

    let input = (*input).clone();
    {
        let seed_tile = &input[&map[0].unwrap()];
        let right_tile = &input[&map[1].unwrap()];
        let right_1_tile = &input[&map[2].unwrap()];
        let bottom_tile = &input[&map[edge as usize].unwrap()];

        let right_set = seed_tile
            .set
            .intersection(&right_tile.set)
            .collect::<HashSet<_>>();
        let right_1_set = right_tile
            .set
            .intersection(&right_1_tile.set)
            .collect::<HashSet<_>>();
        let bottom_set = seed_tile
            .set
            .intersection(&bottom_tile.set)
            .collect::<HashSet<_>>();

        println!("seed\n{:?}", seed_tile);
        println!("right\n{:?}", right_tile);
        println!("right 1\n{:?}", right_1_tile);
        println!("bottom\n{:?}", bottom_tile);

        println!("right set: {:?}", right_set);
        println!("right 1 set: {:?}", right_1_set);
        println!("bottom set: {:?}", bottom_set);
    }

    todo!()
}

pub fn part_1() -> u128 {
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
        static ref INPUT: Tiles = include_str!("../input-example")
            .trim()
            .parse()
            .expect("invalid input");
    }

    #[test]
    fn same_results_part_1() {
        assert_eq!(solve_1(&INPUT), 20899048083289);
    }

    #[test]
    fn same_results_part_2() {
        assert_eq!(solve_2(&INPUT), 273);
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
