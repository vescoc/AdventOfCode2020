pub struct BitSet {
    data: Vec<u64>,
    size: usize,
}

impl BitSet {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
            size,
        }
    }

    pub fn get(&self, index: usize) -> bool {
        assert!(index < self.size, "invalid index {} > {}", index, self.size);
        self.data[index / std::mem::size_of::<u64>()] & 1 << (index % std::mem::size_of::<u64>()) != 0
    }

    pub fn set(&mut self, index: usize, b: bool) {
        assert!(index < self.size, "invalid index {} > {}", index, self.size);
        let mut data = self.data[index / std::mem::size_of::<u64>()];
        if b {
            data |= 1 << (index % std::mem::size_of::<u64>());
        } else {
            data &= !(1 << (index % std::mem::size_of::<u64>()));
        }
        self.data[index / std::mem::size_of::<u64>()] = data;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set() {
        let mut set = BitSet::new(1024);
        set.set(129, true);

        assert!(set.get(129));

        set.set(129, false);
        assert!(!set.get(129));
    }

    #[test]
    fn get() {
        let set = BitSet::new(1024);
        assert!(!set.get(129));
    }
}
