#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;

lazy_static! {
    static ref INPUT: Data<String> = include_str!("../input").parse().expect("invalid input");
}

#[derive(Clone)]
struct Data<T>(Vec<(HashSet<T>, HashSet<T>)>);

impl <T: Eq + std::hash::Hash + Clone> Data<T> {
    fn reduce(&mut self) -> Option<HashSet<(T, T)>> {
        let (mut r_ingredients, mut r_allergens, mut res) =
            (HashSet::new(), HashSet::new(), HashSet::new());

        for (ingredients, allergens) in &self.0 {
            assert!(ingredients.len() >= allergens.len());

            if ingredients.len() == allergens.len() {
                r_ingredients = r_ingredients.union(&ingredients).cloned().collect();
                r_allergens = r_allergens.union(&allergens).cloned().collect();

                if ingredients.len() == 1 {
                    res.insert((
                        ingredients.iter().next().unwrap().to_owned(),
                        allergens.iter().next().unwrap().to_owned(),
                    ));
                }
            }
        }

        if r_ingredients.is_empty() && self.0.len() > 2 {
            let mut data = Vec::new();
            for (i, (a_ingredients, a_allergens)) in self.0[0..self.0.len() - 2].iter().enumerate()
            {
                for (b_ingredients, b_allergens) in &self.0[i..] {
                    let allergens = a_allergens
                        .intersection(b_allergens)
                        .cloned()
                        .collect::<HashSet<_>>();
                    if !allergens.is_empty() {
                        let ingredients = a_ingredients
                            .intersection(b_ingredients)
                            .cloned()
                            .collect::<HashSet<_>>();
                        if !data.contains(&(ingredients.to_owned(), allergens.to_owned())) {
                            data.push((ingredients, allergens));
                        }
                    }
                }
            }

            if let Some(r) = Data(data).reduce() {
                r.iter().for_each(|(r_i, r_a)| {
                    r_ingredients.insert(r_i.to_owned());
                    r_allergens.insert(r_a.to_owned());
                });
                res = r;
            }
        }

        if !r_ingredients.is_empty() {
            for (ingredients, allergens) in self.0.iter_mut() {
                *ingredients = ingredients.difference(&r_ingredients).cloned().collect();
                *allergens = allergens.difference(&r_allergens).cloned().collect();
            }
            Some(res)
        } else {
            None
        }
    }
}

impl std::str::FromStr for Data<String> {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            input
                .lines()
                .map(|line| {
                    let mut parts = line.split(" (contains ");
                    match (parts.next(), parts.next()) {
                        (Some(ingredients), Some(allergens)) => Ok((
                            ingredients
                                .split_ascii_whitespace()
                                .map(str::to_string)
                                .collect(),
                            allergens[0..allergens.len() - 1]
                                .split(", ")
                                .map(str::to_string)
                                .collect(),
                        )),
                        _ => Err(format!("invalid line: {}", line)),
                    }
                })
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl<T> std::ops::Deref for Data<T> {
    type Target = Vec<(HashSet<T>, HashSet<T>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn solve(input: &Data<String>) -> (usize, String) {
    let mut input = (*input).clone();
    
    let mut v = Vec::new();
    while let Some(set) = input.reduce() {
        set.iter().cloned().for_each(|s| v.push(s));
    }

    v.sort_by_key(|(_, a)| a.to_owned());

    (
        input.iter().map(|(i, _)| i).flatten().count(),
        v.iter()
            .map(|(i, _)| i)
            .cloned()
            .collect::<Vec<_>>()
            .join(","),
    )    
}

pub fn part() -> (usize, String) {
    solve(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    lazy_static! {
        static ref INPUT: Data<String> = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"
            .parse()
            .expect("invalid input");
    }

    #[test]
    fn same_results_part_1() {
        assert_eq!(solve(&INPUT).0, 5);
    }

    #[test]
    fn same_results_part_2() {
        assert_eq!(solve(&INPUT).1, "mxmxvkd,sqjhc,fvjkl");
    }

    #[bench]
    fn bench_solve_test(b: &mut Bencher) {
        b.iter(|| solve(&INPUT));
    }

    #[bench]
    #[ignore]
    fn bench_solve(b: &mut Bencher) {
        b.iter(part);
    }
}
