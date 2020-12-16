#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

#[cfg(not(feature = "optimized"))]
use std::collections::HashSet;

#[cfg(feature = "optimized")]
use std::collections::HashMap;

lazy_static! {
    static ref INPUT: Note = include_str!("../input").parse().expect("invalid input");
}

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<(u64, u64)>,
}

impl Rule {
    fn is_valid(&self, value: u64) -> bool {
        self.ranges.iter().any(|(a, b)| value >= *a && value <= *b)
    }
}

impl std::str::FromStr for Rule {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split(':');

        let name = parts
            .next()
            .ok_or_else(|| "no name".to_string())?
            .to_string();

        let ranges = parts
            .next()
            .map(|part| {
                part.trim()
                    .split(" or ")
                    .map(|range| {
                        range
                            .split('-')
                            .map(|p| p.parse().map_err(|_| format!("invalid number: {}", p)))
                            .collect::<Result<Vec<u64>, _>>()
                            .and_then(|v| {
                                if v.len() == 2 {
                                    Ok((v[0], v[1]))
                                } else {
                                    Err("invalid format".to_string())
                                }
                            })
                    })
                    .collect::<Result<Vec<(u64, u64)>, _>>()
            })
            .ok_or_else(|| "no ranges".to_string())??;

        Ok(Rule { name, ranges })
    }
}

#[derive(Debug)]
struct Note {
    rules: Vec<Rule>,
    your_ticket: Vec<u64>,
    nearby_tickets: Vec<Vec<u64>>,
}

impl std::str::FromStr for Note {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split("\n\n");

        let rules = parts
            .next()
            .ok_or_else(|| "invalid rules".to_string())
            .and_then(|part| {
                part.split('\n')
                    .map(|line| line.parse())
                    .collect::<Result<_, _>>()
            })?;

        let your_ticket = parts
            .next()
            .ok_or_else(|| "invalid your ticket".to_string())
            .and_then(|part| {
                part.lines()
                    .nth(1)
                    .ok_or_else(|| "invalid your ticket".to_string())
                    .and_then(|line| {
                        line.split(',')
                            .map(|v| v.parse().map_err(|_| format!("invalid number: {}", v)))
                            .collect::<Result<_, _>>()
                    })
            })?;

        let nearby_tickets = parts
            .next()
            .ok_or_else(|| "invalid nearby tickets".to_string())
            .and_then(|part| {
                part.lines()
                    .skip(1)
                    .map(|line| {
                        line.split(',')
                            .map(|v| v.parse().map_err(|_| format!("invalid number: {}", v)))
                            .collect::<Result<_, _>>()
                    })
                    .collect::<Result<_, _>>()
            })?;

        Ok(Note {
            rules,
            your_ticket,
            nearby_tickets,
        })
    }
}

fn solve_1(note: &Note) -> u64 {
    note.nearby_tickets
        .iter()
        .filter_map(|ticket| {
            ticket
                .iter()
                .try_fold(None, |_, value| {
                    if note.rules.iter().any(|rule| rule.is_valid(*value)) {
                        Ok(None)
                    } else {
                        Err(value)
                    }
                })
                .unwrap_or_else(Some)
        })
        .sum()
}

#[cfg(not(feature = "optimized"))]
fn solve_2(note: &Note) -> Vec<String> {
    let sets: Vec<HashSet<String>> = std::iter::once(&note.your_ticket)
        .chain(note.nearby_tickets.iter())
        .filter_map(|ticket| {
            let ticket_set = ticket
                .iter()
                .map(|value| {
                    note.rules
                        .iter()
                        .filter_map(|rule| {
                            if rule.is_valid(*value) {
                                Some(rule.name.clone())
                            } else {
                                None
                            }
                        })
                        .collect::<HashSet<_>>()
                })
                .collect::<Vec<_>>();
            if ticket_set.iter().any(|set| set.is_empty()) {
                None
            } else {
                Some(ticket_set)
            }
        })
        .fold(None, |sets: Option<Vec<HashSet<_>>>, ticket_sets| {
            sets.map(|sets| {
                sets.iter()
                    .zip(ticket_sets.iter())
                    .map(|(s1, s2)| s1.intersection(s2).cloned().collect())
                    .collect()
            })
            .or(Some(ticket_sets))
        })
        .unwrap();

    let (mut reduced, mut current): (Vec<_>, Vec<_>) = (
        Vec::with_capacity(sets.len()),
        sets.into_iter().enumerate().collect(),
    );
    while !current.is_empty() {
        let (mut singles, unknown): (Vec<_>, Vec<_>) =
            current.into_iter().partition(|(_, set)| set.len() == 1);

        assert!(!singles.is_empty());

        let all_singles = singles.iter().fold(HashSet::new(), |set, (_, s)| {
            set.union(s).cloned().collect()
        });

        current = unknown
            .iter()
            .map(|(i, set)| (*i, set.difference(&all_singles).cloned().collect()))
            .collect();
        reduced.append(&mut singles);
    }

    reduced.sort_by_key(|(i, _)| *i);

    reduced
        .iter()
        .map(|(_, set)| set.iter().cloned().next().unwrap())
        .collect()
}

#[cfg(feature = "optimized")]
fn solve_2(note: &Note) -> Vec<String> {
    let mut current_id = 1u64;
    let mut name2id = HashMap::with_capacity(64);
    let mut id2name = HashMap::with_capacity(64);

    let mut get_id = |key: &String| -> u64 {
        *name2id.entry(key.clone()).or_insert_with(|| {
            let k = current_id;
            id2name.insert(k, key.clone());
            current_id <<= 1;
            k
        })
    };

    let sets: Vec<u64> = std::iter::once(&note.your_ticket)
        .chain(note.nearby_tickets.iter())
        .filter_map(|ticket| {
            let ticket_set = ticket
                .iter()
                .map(|value| {
                    let mut set = 0u64;
                    for rule in &note.rules {
                        if rule.is_valid(*value) {
                            set |= get_id(&rule.name);
                        }
                    }

                    set
                })
                .collect::<Vec<_>>();
            if ticket_set.iter().any(|set| set.count_ones() == 0) {
                None
            } else {
                Some(ticket_set)
            }
        })
        .fold(None, |sets: Option<Vec<u64>>, ticket_sets| {
            sets.map(|sets| {
                sets.iter()
                    .zip(ticket_sets.iter())
                    .map(|(s1, s2)| s1 & s2)
                    .collect()
            })
            .or(Some(ticket_sets))
        })
        .unwrap();

    let (mut reduced, mut current): (Vec<_>, Vec<_>) = (
        Vec::with_capacity(sets.len()),
        sets.into_iter().enumerate().collect(),
    );
    while !current.is_empty() {
        let (mut singles, unknown): (Vec<_>, Vec<_>) = current
            .into_iter()
            .partition(|(_, set)| set.count_ones() == 1);

        assert!(!singles.is_empty());

        let all_singles = singles.iter().fold(0u64, |set, (_, s)| set | s);

        current = unknown
            .iter()
            .map(|(i, set)| (*i, set & !all_singles))
            .collect();
        reduced.append(&mut singles);
    }

    reduced.sort_by_key(|(i, _)| *i);

    reduced
        .iter()
        .map(|(_, set)| id2name[set].clone())
        .collect()
}

pub fn part_1() -> u64 {
    solve_1(&INPUT)
}

pub fn part_2() -> u64 {
    let fields = solve_2(&INPUT);

    fields
        .iter()
        .enumerate()
        .filter_map(|(i, name)| {
            if name.starts_with("departure") {
                Some(INPUT.your_ticket[i])
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    lazy_static! {
        static ref INPUT: Note = r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            .parse()
            .expect("invalid input");
    }

    #[test]
    fn same_results_part_1() {
        assert_eq!(solve_1(&INPUT), 71);
    }

    #[test]
    fn same_results_part_2() {
        assert_eq!(
            solve_2(&INPUT),
            vec!["row".to_string(), "class".to_string(), "seat".to_string()]
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
