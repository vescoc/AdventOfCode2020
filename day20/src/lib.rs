#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet, VecDeque};

mod tiles;
use tiles::*;

mod combination2b;
use combination2b::*;

lazy_static! {
    static ref INPUT: Tiles = include_str!("../input")
        .trim()
        .parse()
        .expect("invalid input");
}

macro_rules! check {
    ($msg:expr, $set:expr, $tile:expr, $i:expr) => {
        assert!(
            $set.contains(&$tile.edge_vec[$i].0) && $set.contains(&$tile.edge_vec[$i].1),
            "{}\n{:?}",
            $msg,
            $tile
        );
    };
}

fn solve_1(input: &Tiles) -> u128 {
    input
        .iter()
        .combination2()
        .filter_map(|((a_id, a_tile), (b_id, b_tile))| {
            let i = a_tile
                .edge_set
                .intersection(&b_tile.edge_set)
                .collect::<HashSet<_>>();
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
                let i = a_tile
                    .edge_set
                    .intersection(&b_tile.edge_set)
                    .collect::<HashSet<_>>();
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

    let mut input = (0..edge)
        .map(|y| {
            (0..edge)
                .map(|x| input[&map[(x + y * edge) as usize].unwrap()].to_owned())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // fix rows
    for (y, row) in input.iter_mut().enumerate() {
        let mut x = 0;
        let mut current = &mut row[..];
        while current.len() > 1 {
            let (a, r) = current.split_first_mut().unwrap();

            let set = a
                .edge_set
                .intersection(&r[0].edge_set)
                .copied()
                .collect::<HashSet<_>>();

            {
                // rotate a, if possible
                let p = a.find(&set).unwrap();
                if p != RIGHT_INDEX {
                    if x == 0 {
                        let angle = RIGHT_INDEX as isize - p as isize;
                        a.rotate(angle);
                        check!("a", set, a, RIGHT_INDEX);
                    } else {
                        panic!("cannot rotate ({}, {})!", x, y);
                    }
                }
            }

            {
                // rotate b
                let p = r[0].find(&set).unwrap();
                if p != LEFT_INDEX {
                    let angle = LEFT_INDEX as isize - p as isize;
                    r[0].rotate(angle);
                    check!("b", set, r[0], LEFT_INDEX);
                }
            }

            if a.edge_vec[RIGHT_INDEX] != r[0].edge_vec[LEFT_INDEX] {
                r[0].flip_h();
            }

            assert_eq!(
                a.edge_vec[RIGHT_INDEX], r[0].edge_vec[LEFT_INDEX],
                "\ntile a\n{:?}\ntile b\n{:?}",
                a, r[0]
            );

            x += 1;
            current = r;
        }
    }

    // fix columns
    for x in 0..input.len() {
        let mut y = 0;
        let mut current = &mut input[..];
        while current.len() > 1 {
            let (a, r) = current.split_first_mut().unwrap();

            let set = a[x]
                .edge_set
                .intersection(&r[0][x].edge_set)
                .copied()
                .collect::<HashSet<_>>();

            {
                // flip a, if possible
                let p = a[x].find(&set).unwrap();
                if p != BOTTOM_INDEX {
                    if y == 0 {
                        a[x].flip_v();
                    } else {
                        panic!("cannot flip ({}, {})!", x, y);
                    }
                }
            }

            {
                // flip b
                let p = r[0][x].find(&set).unwrap();
                if p != TOP_INDEX {
                    r[0][x].flip_v();
                }
            }

            assert_eq!(
                a[x].edge_vec[BOTTOM_INDEX], r[0][x].edge_vec[TOP_INDEX],
                "\n({}, {})\ntile a\n{:?}\ntile b\n{:?}",
                x, y, a[x], r[0][x]
            );

            y += 1;
            current = r;
        }
    }

    // check
    for (y, row) in input.iter().enumerate() {
        for (x, p) in row.windows(2).enumerate() {
            assert_eq!(
                p[0].edge_vec[RIGHT_INDEX], p[1].edge_vec[LEFT_INDEX],
                "\n({}, {})\ntile a\n{:?}\ntile b\n{:?}",
                x, y, p[0], p[1]
            );
        }
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
    #[ignore]
    fn bench_part_2(b: &mut Bencher) {
        b.iter(part_2);
    }
}
