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

#[cfg(debug_assertions)]
macro_rules! print_mask {
    ($e:expr) => {
        println!(stringify!($e));
        for i in $e {
            println!("{:0128b}", i);
        }
    };
}

#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($($e:expr),*) => {};
}

#[cfg(not(debug_assertions))]
macro_rules! print_mask {
    ($e:expr) => {};
}

lazy_static! {
    static ref INPUT: Tiles<u32> = include_str!("../input")
        .trim()
        .parse()
        .expect("invalid input");
    static ref MONSTER: &'static str = r"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";
    static ref MONSTER_PATTERN: Vec<u128> = MONSTER
        .lines()
        .map(|line| line
            .chars()
            .fold(0, |v, c| if c == '#' { (v << 1) | 1 } else { v << 1 }))
        .collect();
    static ref MONSTER_WIDTH: usize = MONSTER.lines().next().unwrap().len();
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

fn assemble_tile(input: &Tiles<u32>) -> (Tile<(), TileOptionalNop>, usize, usize, usize) {
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
            let seed = *neighbors
                .iter()
                .filter_map(|(id, v)| if v.len() == 2 { Some(*id) } else { None })
                .next()
                .unwrap();

            let mut seed_tile = input[&seed].to_owned();

            let mut n = neighbors[&seed].iter();

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

            set(&mut map, &mut tiles, &mut found, (0, 0), seed, seed_tile);
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
            if get(&map, (x, y)).is_some() {
                continue;
            }

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

            if x + 1 < edge {
                queue.push_back((x + 1, y));
            }
            if y + 1 < edge {
                queue.push_back((x, y + 1));
            }
        }
    }

    // now map & tiles are a good image, but unknown rotation / flip
    // removing borders...

    let edge = edge as usize;
    let h = input.values().next().unwrap().image.len() - 2;
    let w = input.values().next().unwrap().image[0].len() - 2;
    debug!("making mega tile: ({}, {})", edge * w, edge * h);

    let mut image = Vec::new();
    image.resize_with(edge * h, Vec::new);

    let mut row = 0;
    for y in 0..edge {
        for x in 0..edge {
            let tile = &tiles[&map[x + y * edge].unwrap()];
            for (i, v) in tile.image[1..tile.image.len() - 1].iter().enumerate() {
                image[row + i].append(&mut v[1..v.len() - 1].to_owned());
            }
        }
        row += h;
    }

    assert_eq!(image.len(), edge * h, "invalid rows");
    assert_eq!(image[0].len(), edge * w, "invalid cols");

    let tile: Tile<(), TileOptionalNop> = Tile::new_from_image(image);
    debug!("mega tile\n{:?}", tile);

    (tile, edge, h, w)
}

fn solve_2(input: &Tiles<u32>) -> u32 {
    let (tile, edge, h, _w) = assemble_tile(input);

    let total_water_roughness = tile.get_mask().iter().map(|v| v.count_ones()).sum::<u32>();

    print_mask!(tile.get_mask());
    print_mask!(MONSTER_PATTERN.iter());

    let flips: Vec<fn(&mut Tile<_, _>)> = vec![flip_none, flip_h, flip_v];
    for r in 0..4 {
        let mut tile = tile.to_owned();
        tile.rotate(r);

        for f in &flips {
            let mut tile = tile.to_owned();
            f(&mut tile);

            let monster_roughness =
                check_pattern(&tile.get_mask(), edge * h, &MONSTER_PATTERN, *MONSTER_WIDTH);
            if monster_roughness != 0 {
                return total_water_roughness - monster_roughness;
            }
        }
    }

    unreachable!()
}

fn flip_none<T, O: TileOptional<T>>(_tile: &mut Tile<T, O>) {
    // none
}

fn flip_h<T, O: TileOptional<T>>(tile: &mut Tile<T, O>) {
    tile.flip_h();
}

fn flip_v<T, O: TileOptional<T>>(tile: &mut Tile<T, O>) {
    tile.flip_v();
}

fn check_pattern(
    image: &[u128],
    image_width: usize,
    pattern: &[u128],
    pattern_width: usize,
) -> u32 {
    let pattern_ones = pattern.iter().map(|v| v.count_ones()).collect::<Vec<_>>();
    let image_height = image.len();
    let pattern_height = pattern.len();

    let mut count = 0;
    let mut row = 0;
    while row < image_height - pattern_height + 1 {
        let mut partial_count = 0;
        let mut s = 0;
        while s < image_width - pattern_width + 1 {
            if image[row..row + pattern_height]
                .iter()
                .enumerate()
                .all(|(i, row)| (row & (pattern[i] << s)).count_ones() == pattern_ones[i])
            {
                debug!("hit at ({}, {})", s, row);
                partial_count += 1;
            }
            s += 1;
        }
        if partial_count > 0 {
            count += partial_count;
        }
        row += 1;
    }

    if count != 0 {
        count * pattern_ones.into_iter().sum::<u32>()
    } else {
        0
    }
}

pub fn part_1() -> u128 {
    solve_1(&INPUT)
}

pub fn part_2() -> u32 {
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
        static ref INPUT_RESULT: Tile<(), TileOptionalNop> =
            Tile::new_from_lines(include_str!("../input-example-result").trim().lines())
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

    #[test]
    fn same_results_part_2_partial() {
        let tile = assemble_tile(&INPUT).0;

        debug!("assembled tile\n{:?}", tile);
        debug!("target tile\n{:?}", *INPUT_RESULT);

        assert_eq!(tile.image.len(), INPUT_RESULT.image.len(), "invalid height");
        assert_eq!(
            tile.image[0].len(),
            INPUT_RESULT.image[0].len(),
            "invalid height"
        );

        let flips: Vec<fn(&mut Tile<_, _>)> = vec![flip_none, flip_h, flip_v];
        for f in flips {
            let mut tile = tile.to_owned();

            for i in 0..4 {
                f(&mut tile);
                tile.rotate(i);

                if tile.image == INPUT_RESULT.image {
                    return;
                }
            }
        }

        unreachable!();
    }

    #[test]
    fn test_check_pattern() {
        assert_eq!(
            check_pattern(
                &MONSTER_PATTERN,
                *MONSTER_WIDTH,
                &MONSTER_PATTERN,
                *MONSTER_WIDTH
            ),
            15
        );
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
