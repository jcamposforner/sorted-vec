use std::cmp::Ordering;
use crate::AddResult;

#[derive(Default, Debug)]
pub(crate) struct Bucket<T: PartialOrd + Ord> {
    pub(crate) data: Vec<T>,
}

impl<T: PartialOrd + Ord> Bucket<T> {
    pub fn new(data: Vec<T>) -> Self {
        Bucket { data }
    }

    pub fn empty() -> Self {
        Bucket { data: Vec::new() }
    }

    pub fn insert(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub(crate) fn split(&mut self) -> Bucket<T> {
        let curr_len = self.data.len();
        let at = curr_len / 2;

        let other_len = self.data.len() - at;
        let mut other = Vec::with_capacity(curr_len);

        unsafe {
            self.data.set_len(at);
            other.set_len(other_len);

            std::ptr::copy_nonoverlapping(self.data.as_ptr().add(at), other.as_mut_ptr(), other.len());
        }

        Bucket { data: other }
    }

    pub fn add(&mut self, item: T) -> AddResult {
        match self.data.binary_search(&item) {
            Ok(idx) => AddResult::Duplicated(idx),
            Err(idx) => {
                self.data.insert(idx, item);
                AddResult::Added(idx)
            },
        }
    }

    pub fn item_compare(&self, item: &T) -> Ordering {
        let first_item = match self.data.first() {
            Some(f) => f,
            None => return Ordering::Equal,
        };

        let last_item = match self.data.last() {
            Some(l) => l,
            None => return Ordering::Equal,
        };

        if item < first_item {
            return Ordering::Greater;
        }

        if last_item < item {
            return Ordering::Less;
        }

        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_insert() {
        let mut bucket = Bucket::empty();
        bucket.insert(1);
        bucket.insert(2);
        bucket.insert(3);
        bucket.insert(4);
        bucket.insert(5);

        assert_eq!(bucket.data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bucket_split() {
        let mut bucket = Bucket::new(vec![1, 2, 3, 4, 5]);
        let new_bucket = bucket.split();

        assert_eq!(bucket.data, vec![1, 2]);
        assert_eq!(new_bucket.data, vec![3, 4, 5]);
    }

    #[test]
    fn bucket_empty_has_no_elements() {
        let bucket = Bucket::<i32>::empty();
        assert_eq!(bucket.len(), 0);
    }

    #[test]
    fn bucket_new_has_correct_elements() {
        let bucket = Bucket::new(vec![1, 2, 3]);
        assert_eq!(bucket.data, vec![1, 2, 3]);
    }

    #[test]
    fn bucket_add_inserts_in_sorted_order() {
        let mut bucket = Bucket::empty();
        bucket.add(3);
        bucket.add(1);
        bucket.add(2);
        assert_eq!(bucket.data, vec![1, 2, 3]);
    }

    #[test]
    fn bucket_add_returns_correct_result() {
        let mut bucket = Bucket::empty();
        assert_eq!(bucket.add(1), AddResult::Added(0));
        assert_eq!(bucket.add(1), AddResult::Duplicated(0));
    }

    #[test]
    fn bucket_split_on_empty_bucket() {
        let mut bucket = Bucket::<i32>::empty();
        let new_bucket = bucket.split();
        assert_eq!(bucket.data, vec![]);
        assert_eq!(new_bucket.data, vec![]);
    }

    #[test]
    fn bucket_item_compare_less_than_first() {
        let bucket = Bucket::new(vec![2, 3, 4]);
        assert_eq!(bucket.item_compare(&1), Ordering::Greater);
    }

    #[test]
    fn bucket_item_compare_greater_than_last() {
        let bucket = Bucket::new(vec![2, 3, 4]);
        assert_eq!(bucket.item_compare(&5), Ordering::Less);
    }

    #[test]
    fn bucket_item_compare_within_range() {
        let bucket = Bucket::new(vec![2, 3, 4]);
        assert_eq!(bucket.item_compare(&3), Ordering::Equal);
    }
}