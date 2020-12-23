use super::*;

macro_rules! crapcupsarray {
    [$i:tt] => {
pub struct CrapCupsIter {
    labels: [u32; $i],
    current_cup_index: usize,
    tmp: [u32; $i - 3],
}

impl CrapCups for [u32; $i] {
    type I = CrapCupsIter;
    
    fn crap_cups(&self) -> Self::I {
        CrapCupsIter {
            labels: self.to_owned(),
            current_cup_index: 0,
            tmp: [0; $i - 3],
        }
    }
}

impl Iterator for CrapCupsIter {
    type Item = CrapCupsValue;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.labels.to_owned();

        let find = |data: &[u32], value| {
            data.iter()
                .enumerate()
                .find_map(|(i, v)| if *v == value { Some(i) } else { None })
                .unwrap()
        };

        let current_cup = current[self.current_cup_index];

        let destination_cup = {
            let mut destination_cup = current_cup - 1;
            loop {
                if destination_cup == 0 {
                    destination_cup = current.len() as u32;
                }

                if (1..4).any(|i| {
                    current[(self.current_cup_index + i) % current.len()] == destination_cup
                }) {
                    destination_cup =
                        (destination_cup + current.len() as u32 - 1) % current.len() as u32;
                } else {
                    break destination_cup;
                }
            }
        };

        let (mut d, mut c) = (0, 0);
        for i in 0..current.len() {
            if (1..4).any(|s| (self.current_cup_index + s) % current.len() == i) {
                c += 1;
            } else {
                self.tmp[d] = current[c];
                d += 1;
                c += 1;
            }
        }

        let destination_cup_index = find(&self.tmp, destination_cup);
        let current_cup_index = find(&self.tmp, current_cup);

        let (mut d, mut s) = (self.current_cup_index, current_cup_index);
        for _ in 0..current.len() - 3 {
            if s == destination_cup_index {
                self.labels[d] = self.tmp[s];
                self.labels[(d + 1) % current.len()] =
                    current[(self.current_cup_index + 1) % current.len()];
                self.labels[(d + 2) % current.len()] =
                    current[(self.current_cup_index + 2) % current.len()];
                self.labels[(d + 3) % current.len()] =
                    current[(self.current_cup_index + 3) % current.len()];

                d = (d + 4) % current.len();
                s = (s + 1) % (current.len() - 3);
            } else {
                self.labels[d] = self.tmp[s];

                d = (d + 1) % current.len();
                s = (s + 1) % (current.len() - 3);
            }
        }

        self.current_cup_index = (self.current_cup_index + 1) % current.len();

        Some(CrapCupsValue(current))
    }
}

pub struct CrapCupsValue([u32; $i]);

impl std::fmt::Debug for CrapCupsValue {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("{:?}: {}", self.0, &self.to_string()))
    }
}

impl ToString for CrapCupsValue {
    fn to_string(&self) -> String {
        let index = self
            .0
            .iter()
            .enumerate()
            .find_map(|(i, v)| if *v == 1 { Some(i) } else { None })
            .unwrap();
        (1..$i)
            .map(|i| (self.0[(i + index) % self.0.len()] + '0' as u32) as u8 as char)
            .collect()
    }
}
    }
}

mod crapcupsarray9 {
    use super::*;
    
    crapcupsarray![9];
}

#[allow(non_snake_case)]
mod crapcupsarray1M {
    use super::*;
    
    crapcupsarray![10_000];

    impl CrapCups for Vec<u32> {
        type I = CrapCupsIter;

        fn crap_cups(&self) -> Self::I {
            let mut v = [0; 10_000];

            self.iter().enumerate().for_each(|(i, e)| { v[i] = *e; });
            
            CrapCupsIter {
                labels: v,
                current_cup_index: 0,
                tmp: [0; 10_000 - 3],
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_1: [u32; 9] = parse("389125467");
    }

    #[test]
    fn same_results_example_1_1() {
        let value = EXAMPLE_1.crap_cups().next().unwrap();

        assert_eq!(value.to_string(), "25467389");
    }

    #[test]
    fn same_results_example_1_2() {
        let value = EXAMPLE_1.crap_cups().nth(1).unwrap();

        assert_eq!(value.to_string(), "54673289");
    }

    #[test]
    fn same_results_example_1_3() {
        let value = EXAMPLE_1.crap_cups().nth(2).unwrap();

        assert_eq!(value.to_string(), "32546789");
    }

    #[test]
    fn same_results_example_1_4() {
        let value = EXAMPLE_1.crap_cups().nth(3).unwrap();

        assert_eq!(value.to_string(), "34672589");
    }

    #[test]
    fn same_results_example_1_5() {
        let value = EXAMPLE_1.crap_cups().nth(4).unwrap();

        assert_eq!(value.to_string(), "32584679");
    }

    #[test]
    fn same_results_example_1_10() {
        assert_eq!(
            EXAMPLE_1.crap_cups().nth(10).unwrap().to_string(),
            "92658374"
        );
    }
}
