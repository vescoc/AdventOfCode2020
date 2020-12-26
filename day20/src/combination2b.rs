pub struct Combination2<T> {
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

pub trait Combination2B: IntoIterator
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
