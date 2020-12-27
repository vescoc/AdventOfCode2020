#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet, VecDeque};

mod tiles;
use tiles::*;

mod combination2b;
use combination2b::*;

#[cfg(debug_assertions)]
macro_rules! debug {
    ($($e:expr),*) => { println!($($e),*); }
}

#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($($e:expr),*) => {};
}

lazy_static! {
    static ref INPUT: Tiles<u32> = include_str!("../input")
        .trim()
        .parse()
        .expect("invalid input");
}

fn solve_1(input: &Tiles<u32>) -> u128 {
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

fn solve_2(input: &Tiles<u32>) -> usize {
    let size = input.len();
    let edge = (size as f32).sqrt() as i128;

    let mut map: Vec<Option<u128>> = vec![None; size];
    let mut tiles: HashMap<u128, Tile<u32, TileOptionalU32>> = HashMap::new();

    let get = |map: &Vec<Option<u128>>, (x, y): (i128, i128)| {
        if x >= 0 && x < edge && y >= 0 && y < edge {
            map[(x + y * edge) as usize]
        } else {
            None
        }
    };

    // populate tiles
    {
        // working cell
        let mut queue = VecDeque::new();

        // found tiles
        let mut found: HashSet<u128> = HashSet::new();

        // neighbors
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
                map.entry(a_id).or_insert_with(HashSet::new).insert(*b_id);
                map.entry(b_id).or_insert_with(HashSet::new).insert(*a_id);
                map
            });

        let set = |map: &mut Vec<Option<u128>>,
                   tiles: &mut HashMap<u128, Tile<u32, TileOptionalU32>>,
                   found: &mut HashSet<u128>,
                   (x, y): (i128, i128),
                   v,
                   tile| {
            map[(x + y * edge) as usize] = Some(v);
            tiles.insert(v, tile);
            found.insert(v);
        };

        // find and fix [0, 0], [0, 1] and [1, 0]
        {
            let seed = neighbors
                .iter()
                .filter_map(|(id, v)| if v.len() == 2 { Some(*id) } else { None })
                .next()
                .unwrap();

            let mut seed_tile = input[seed].to_owned();

            let mut n = neighbors[seed].iter();

            let right = n.next().unwrap();
            let bottom = n.next().unwrap();

            assert!(n.next().is_none());

            let mut right_tile = input[right].to_owned();
            let mut bottom_tile = input[bottom].to_owned();

            debug!("seed tile: {}\n{:?}", seed, seed_tile);
            debug!("right tile: {}\n{:?}", right, right_tile);
            debug!("bottom tile: {}\n{:?}", bottom, bottom_tile);

            // fix sides
            {
                let right_edge = seed_tile
                    .edge_set
                    .intersection(&right_tile.edge_set)
                    .copied()
                    .collect();
                let bottom_edge = seed_tile
                    .edge_set
                    .intersection(&bottom_tile.edge_set)
                    .copied()
                    .collect();

                debug!("right edge: {:?}", right_edge);
                debug!("bottom edge: {:?}", bottom_edge);
                debug!();

                let right_rotation = right_tile.find(&right_edge).unwrap();
                if right_rotation != LEFT_INDEX {
                    let angle = right_rotation as isize - LEFT_INDEX as isize;
                    right_tile.rotate(angle);
                    debug!("right tile rotate {}", angle);
                    debug!("{:?}", right_tile);
                }

                let bottom_rotation = bottom_tile.find(&bottom_edge).unwrap();
                if bottom_rotation != TOP_INDEX {
                    let angle = bottom_rotation as isize - TOP_INDEX as isize;
                    bottom_tile.rotate(angle);
                    debug!("bottom tile rotate {}", angle);
                    debug!("{:?}", bottom_tile);
                }

                let seed_rotation = seed_tile.find(&right_edge).unwrap();
                if seed_rotation != RIGHT_INDEX {
                    let angle = seed_rotation as isize - RIGHT_INDEX as isize;
                    seed_tile.rotate(angle);
                    debug!("seed tile rotate {}", angle);
                    debug!("{:?}", seed_tile);
                }

                let seed_flip = seed_tile.find(&bottom_edge).unwrap();
                if seed_flip != BOTTOM_INDEX {
                    assert_eq!(seed_flip, TOP_INDEX);
                    seed_tile.flip_v();
                    debug!("seed tile flip v");
                    debug!("{:?}", seed_tile);
                }

                if seed_tile.edge_vec[RIGHT_INDEX].0 != right_tile.edge_vec[LEFT_INDEX].1 {
                    right_tile.flip_v();
                    debug!("right tile flip v");
                    debug!("{:?}", right_tile);
                }

                if seed_tile.edge_vec[BOTTOM_INDEX].0 != bottom_tile.edge_vec[TOP_INDEX].1 {
                    bottom_tile.flip_h();
                    debug!("bottom tile flip h");
                    debug!("{:?}", bottom_tile);
                }

                debug!();

                debug!("post seed tile\n{:?}", seed_tile);
                debug!("post right tile\n{:?}", right_tile);
                debug!("post bottom tile\n{:?}", bottom_tile);

                assert_eq!(
                    seed_tile.edge_vec[RIGHT_INDEX].0, right_tile.edge_vec[LEFT_INDEX].1,
                    "seed vs right"
                );
                assert_eq!(
                    seed_tile.edge_vec[BOTTOM_INDEX].0, bottom_tile.edge_vec[TOP_INDEX].1,
                    "seed vs bottom"
                );
            }

            set(&mut map, &mut tiles, &mut found, (0, 0), *seed, seed_tile);
            set(&mut map, &mut tiles, &mut found, (1, 0), *right, right_tile);
            set(
                &mut map,
                &mut tiles,
                &mut found,
                (0, 1),
                *bottom,
                bottom_tile,
            );

            queue.push_back((1, 1));
            queue.push_back((0, 2));
            queue.push_back((2, 0));
        }

        while let Some((x, y)) = queue.pop_front() {
            let (left_id, top_id) = (get(&map, (x - 1, y)), get(&map, (x, y - 1)));
            let n = match (left_id, top_id) {
                (Some(id_a), Some(id_b)) => neighbors[&id_a]
                    .intersection(&neighbors[&id_b])
                    .copied()
                    .collect::<HashSet<_>>()
                    .difference(&found)
                    .copied()
                    .collect::<HashSet<_>>(),
                (None, Some(id)) => neighbors[&id]
                    .difference(&found)
                    .copied()
                    .collect::<HashSet<_>>(),
                (Some(id), None) => neighbors[&id]
                    .difference(&found)
                    .copied()
                    .collect::<HashSet<_>>(),
                (None, None) => unreachable!(),
            };

            assert_eq!(n.len(), 1, "({}, {}): {:?}", x, y, n);

            let id = n.iter().next().unwrap();
            let mut tile = input[id].to_owned();
            debug!("\nworking on {}\n{:?}", id, tile);

            // TODO: I think this algo is bad
            // I must rotate & flip tile on one shot if left and top edge are both of them defined

            let top_edge = if let Some(top_id) = top_id {
                let top_tile = &tiles[&top_id];
                let top_edge = top_tile
                    .edge_set
                    .intersection(&tile.edge_set)
                    .copied()
                    .collect::<HashSet<_>>();
                assert_eq!(top_edge.len(), 2, "top_edge invalid");

                debug!("top tile: {}\nedge: {:?}\n{:?}", top_id, top_edge, top_tile);
                assert_eq!(
                    top_tile.find(&top_edge).unwrap(),
                    BOTTOM_INDEX,
                    "({}, {}) top_tile invalid position",
                    x,
                    y
                );

                let top_rotation = tile.find(&top_edge).unwrap();
                if top_rotation != TOP_INDEX {
                    let angle = top_rotation as isize - TOP_INDEX as isize;
                    tile.rotate(angle);
                    debug!("tile rotate {}", angle);
                }

                Some(top_tile.edge_vec[BOTTOM_INDEX].to_owned())
            } else {
                None
            };

            let left_edge = if let Some(left_id) = left_id {
                let left_tile = &tiles[&left_id];
                let left_edge = left_tile
                    .edge_set
                    .intersection(&tile.edge_set)
                    .copied()
                    .collect::<HashSet<_>>();
                assert_eq!(left_edge.len(), 2, "left_edge invalid");

                debug!("left tile: {}\nedge: {:?}\n{:?}", left_id, edge, left_tile);
                assert_eq!(
                    left_tile.find(&left_edge).unwrap(),
                    RIGHT_INDEX,
                    "({}, {}) left_tile invalid position",
                    x,
                    y
                );

                let left_rotation = tile.find(&left_edge).unwrap();
                if left_rotation != LEFT_INDEX {
                    if top_edge.is_some() {
                        assert_eq!(
                            left_rotation, RIGHT_INDEX,
                            "({}, {}), cannot rotate {}",
                            x, y, left_rotation
                        );
                        tile.flip_h();
                        debug!("tile flip h");
                    } else {
                        let angle = left_rotation as isize - LEFT_INDEX as isize;
                        tile.rotate(angle);
                        debug!("tile rotate {}", angle);
                    }
                }

                Some(left_tile.edge_vec[RIGHT_INDEX].to_owned())
            } else {
                None
            };

            if let Some((a, _)) = left_edge {
                if tile.edge_vec[LEFT_INDEX].1 != a {
                    tile.flip_v();
                    debug!("tile flip v");
                }
            }
            if let Some((a, _)) = top_edge {
                if tile.edge_vec[TOP_INDEX].1 != a {
                    tile.flip_h();
                    debug!("tile flip h");
                }
            }

            if let Some((a, _)) = left_edge {
                assert_eq!(
                    a, tile.edge_vec[LEFT_INDEX].1,
                    "({}, {}) invalid left position",
                    x, y
                );
            }
            if let Some((a, _)) = top_edge {
                assert_eq!(
                    a, tile.edge_vec[TOP_INDEX].1,
                    "({}, {}) invalid top position",
                    x, y
                );
            }

            set(&mut map, &mut tiles, &mut found, (x, y), *id, tile);
        }
    }

    // now map & tiles are a good image, but unknown rotation / flip
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
        static ref INPUT: Tiles<u32> = include_str!("../input-example")
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
