#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet, VecDeque};

lazy_static! {
    static ref INPUT: Graph = Graph::new(include_str!("../input"));
}

#[derive(Debug)]
struct Graph {
    nodes: HashSet<String>,
    edges: HashMap<String, HashSet<String>>,
    reverse_edges: HashMap<String, HashSet<String>>,
    weights: HashMap<(String, String), usize>,
}

impl Graph {
    fn new(input: &str) -> Self {
        let (nodes, edges, reverse_edges, weights) = input
            .split('\n')
            .map(|line| {
                let mut parts = line.split(" contain ");
                let node = {
                    let mut p = parts.next().unwrap().split_ascii_whitespace();
                    format!("{} {}", p.next().unwrap(), p.next().unwrap())
                };

                let neighbors: HashSet<(String, usize)> =
                    parts
                        .next()
                        .unwrap()
                        .split(',')
                        .fold(HashSet::new(), |mut v, part| {
                            if part == "no other bags." || part.is_empty() {
                                v
                            } else {
                                let mut parts = part.trim_start().split_ascii_whitespace();
                                let weight = parts.next().unwrap().parse().expect("invalid number");
                                let node =
                                    format!("{} {}", parts.next().unwrap(), parts.next().unwrap());
                                v.insert((node, weight));
                                v
                            }
                        });

                (node, neighbors)
            })
            .fold(
                (
                    HashSet::new(),
                    HashMap::new(),
                    HashMap::new(),
                    HashMap::new(),
                ),
                |(mut nodes, mut edges, mut reverse_edges, mut weights), (node, neighbors)| {
                    nodes.insert(node.clone());

                    for (n, w) in &neighbors {
                        edges
                            .entry(node.clone())
                            .or_insert_with(HashSet::new)
                            .insert(n.clone());
                        reverse_edges
                            .entry(n.clone())
                            .or_insert_with(HashSet::new)
                            .insert(node.clone());
                        weights.insert((node.clone(), n.clone()), *w);
                    }

                    (nodes, edges, reverse_edges, weights)
                },
            );

        Self {
            nodes,
            edges,
            reverse_edges,
            weights,
        }
    }
}

fn solve_1(input: &Graph) -> usize {
    let mut visit = {
        let mut queue = VecDeque::new();
        queue.push_back("shiny gold".to_string());
        queue
    };
    let mut cover = HashSet::new();

    while let Some(current) = visit.pop_front() {
        if let Some(neighbors) = &input.reverse_edges.get(&current) {
            for node in neighbors.iter() {
                if !cover.contains(node) {
                    visit.push_back(node.clone());
                    cover.insert(node.clone());
                }
            }
        }
    }

    cover.len()
}

fn solve_2(input: &Graph) -> usize {
    let mut visit = {
        let mut queue = VecDeque::new();
        queue.push_back("shiny gold".to_string());
        queue
    };
    let mut total: HashMap<String, usize> = HashMap::new();

    while let Some(current) = visit.pop_front() {
        if let Some(neighbors) = &input.edges.get(&current) {
            let mut sum = Some(0);
            for node in neighbors.iter() {
                if let Some(t) = total.get(node) {
                    sum =
                        sum.map(|v| v + (t + 1) * input.weights[&(current.clone(), node.clone())]);
                } else {
                    if !visit.contains(node) {
                        visit.push_back(node.clone());
                    }
                    sum = None;
                }
            }

            if let Some(t) = sum {
                total.insert(current.clone(), t);
            } else if !visit.contains(&current) {
                visit.push_back(current.clone());
            }
        } else {
            total.insert(current.clone(), 0);
        }
    }

    total["shiny gold"]
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
        static ref EXAMPLE_1: Graph = Graph::new(
            r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
        );
        static ref EXAMPLE_2: Graph = Graph::new(
            r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
        );
    }

    #[test]
    fn same_results_part_1() {
        assert_eq!(solve_1(&EXAMPLE_1), 4);
    }

    #[test]
    fn same_results_part_2_1() {
        assert_eq!(solve_2(&EXAMPLE_1), 32);
    }

    #[test]
    fn same_results_part_2_2() {
        assert_eq!(solve_2(&EXAMPLE_2), 126);
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
