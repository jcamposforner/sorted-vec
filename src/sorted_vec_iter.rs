use crate::sorted_vec::SortedVec;

pub struct SortedVecIter<'a, T: PartialOrd + Ord> {
    sorted_vec: &'a SortedVec<T>,
    index: usize,
}

impl<'a, T: PartialOrd + Ord> SortedVecIter<'a, T> {
    pub fn new(sorted_vec: &'a SortedVec<T>) -> Self {
        SortedVecIter {
            sorted_vec,
            index: 0,
        }
    }
}

impl<'a, T: PartialOrd + Ord> ExactSizeIterator for SortedVecIter<'a, T> {
    fn len(&self) -> usize {
        self.sorted_vec.size
    }
}

impl<'a, T: PartialOrd + Ord> Iterator for SortedVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.sorted_vec.size {
            let item = self.sorted_vec.at(self.index);
            self.index += 1;
            item
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_vec_iter() {
        let mut sorted_vec = SortedVec::new(Default::default());
        sorted_vec.insert(1);
        sorted_vec.insert(2);
        sorted_vec.insert(3);
        sorted_vec.insert(4);
        sorted_vec.insert(5);

        let mut iter = sorted_vec.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_sorted_vec_iter_empty() {
        let sorted_vec: SortedVec<usize> = SortedVec::new(Default::default());
        let mut iter = SortedVecIter::new(&sorted_vec);
        assert_eq!(iter.next(), None);
    }
}