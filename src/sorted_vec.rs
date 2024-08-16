use std::cmp::min;
use std::ops::Deref;

use crate::AddResult;
use crate::bucket::Bucket;
use crate::iter::SortedVecIter;

pub struct FindResult {
    bucket_idx: usize,
    item_idx: usize,
}

impl FindResult {
    pub fn new(bucket_idx: usize, item_idx: usize) -> Self {
        FindResult {
            bucket_idx,
            item_idx,
        }
    }

    pub fn bucket_idx(&self) -> usize {
        self.bucket_idx
    }

    pub fn item_idx(&self) -> usize {
        self.item_idx
    }
}

impl Deref for FindResult {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.item_idx
    }
}

#[derive(Debug, Copy, Clone)]
pub struct MaxBucketCapacity(usize);

impl Deref for MaxBucketCapacity {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MaxBucketCapacity {
    pub fn new(size: usize) -> Self {
        if size < 1 {
            panic!("MaxBucketCapacity must be greater than 0");
        }

        MaxBucketCapacity(size)
    }
}

impl Default for MaxBucketCapacity {
    fn default() -> Self {
        MaxBucketCapacity(200)
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct BucketConfiguration {
    max_bucket_capacity: MaxBucketCapacity,
    initial_set_capacity: usize,
}

impl BucketConfiguration {
    pub fn new(max_bucket_capacity: MaxBucketCapacity, initial_set_capacity: usize) -> Self {
        BucketConfiguration {
            max_bucket_capacity,
            initial_set_capacity,
        }
    }
}

#[derive(Default, Debug)]
pub struct SortedVec<T: PartialOrd + Ord> {
    pub(crate) buckets: Vec<Bucket<T>>,
    configuration: BucketConfiguration,
    pub(crate) size: usize
}

impl<T: PartialOrd + Ord> SortedVec<T> {
    pub fn new(configuration: BucketConfiguration) -> Self {
        let mut result = Self::empty(configuration);
        result.buckets.push(Bucket::empty());
        result
    }

    fn empty(configuration: BucketConfiguration) -> Self {
        let buckets = Vec::with_capacity(configuration.initial_set_capacity);

        SortedVec {
            buckets,
            configuration,
            size: 0,
        }
    }

    pub fn insert(&mut self, item: T) {
        let idx = self.find_bucket_index(&item);
        let bucket = &mut self.buckets[idx];

        match bucket.add(item) {
            AddResult::Added(_) => {
                let bucket_len = bucket.len();
                if bucket_len > *self.configuration.max_bucket_capacity {
                    let new_bucket = bucket.split();
                    self.buckets.insert(idx + 1, new_bucket);
                }

                self.size += 1;
            },
            AddResult::Duplicated(_) => {}
        }
    }

    fn find_bucket_index(&self, item: &T) -> usize {
        match self
            .buckets
            .binary_search_by(|bucket| bucket.item_compare(item))
        {
            Ok(idx) => idx,
            Err(idx) => {
                min(idx, self.buckets.len() - 1)
            },
        }
    }

    pub fn iter(&self) -> SortedVecIter<T> {
        SortedVecIter::new(self)
    }

    pub fn at(&self, mut idx: usize) -> Option<&T> {
        for bucket in &self.buckets {
            if idx < bucket.len() {
                return Some(&bucket.data[idx]);
            }

            idx -= bucket.len();
        }

        None
    }

    pub fn remove(&mut self, item: &T) {
        if let Some(FindResult { bucket_idx, item_idx }) = self.find_index(item) {
            let bucket = &mut self.buckets[bucket_idx];
            bucket.data.remove(item_idx);
            if bucket.data.is_empty() {
                self.buckets.remove(bucket_idx);
            }

            self.size -= 1;
        };
    }

    pub fn slice(&self, start: usize, end: usize) -> Vec<&T> {
        let mut result = Vec::new();
        for i in start..end {
            if let Some(item) = self.at(i) {
                result.push(item);
            }
        }

        result
    }

    pub fn find_index(&self, item: &T) -> Option<FindResult> {
        let bucket_idx = self.find_bucket_index(item);
        let bucket = &self.buckets[bucket_idx];

        match bucket.data.binary_search(item) {
            Ok(i) => Some(FindResult::new(bucket_idx, i)),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sorted_vec::{BucketConfiguration, MaxBucketCapacity, SortedVec};

    #[test]
    fn max_bucket_capacity_new_with_valid_size() {
        let capacity = MaxBucketCapacity::new(10);
        assert_eq!(*capacity, 10);
    }

    #[test]
    #[should_panic(expected = "MaxBucketCapacity must be greater than 0")]
    fn max_bucket_capacity_new_with_zero_size() {
        MaxBucketCapacity::new(0);
    }

    #[test]
    fn max_bucket_capacity_default() {
        let capacity = MaxBucketCapacity::default();
        assert_eq!(*capacity, 200);
    }

    #[test]
    fn bucket_configuration_new_with_valid_values() {
        let config = BucketConfiguration::new(MaxBucketCapacity::new(10), 5);
        assert_eq!(*config.max_bucket_capacity, 10);
        assert_eq!(config.initial_set_capacity, 5);
    }

    #[test]
    fn sorted_vec_new_with_configuration() {
        let config = BucketConfiguration::new(MaxBucketCapacity::new(10), 5);
        let sorted_vec: SortedVec<i32> = SortedVec::new(config);
        assert_eq!(sorted_vec.buckets.len(), 1);
        assert_eq!(sorted_vec.size, 0);
    }

    #[test]
    fn sorted_vec_insert_single_element() {
        let config = BucketConfiguration::new(MaxBucketCapacity::new(10), 5);
        let mut sorted_vec: SortedVec<i32> = SortedVec::new(config);
        sorted_vec.insert(5);
        assert_eq!(sorted_vec.size, 1);
        assert_eq!(sorted_vec.at(0), Some(&5));
    }

    #[test]
    fn sorted_vec_insert_multiple_elements() {
        let config = BucketConfiguration::new(MaxBucketCapacity::new(10), 5);
        let mut sorted_vec: SortedVec<i32> = SortedVec::new(config);
        sorted_vec.insert(5);
        sorted_vec.insert(3);
        sorted_vec.insert(8);
        assert_eq!(sorted_vec.size, 3);
        assert_eq!(sorted_vec.at(0), Some(&3));
        assert_eq!(sorted_vec.at(1), Some(&5));
        assert_eq!(sorted_vec.at(2), Some(&8));
    }

    #[test]
    fn sorted_vec_insert_duplicate_element() {
        let config = BucketConfiguration::new(MaxBucketCapacity::new(10), 5);
        let mut sorted_vec: SortedVec<i32> = SortedVec::new(config);
        sorted_vec.insert(5);
        sorted_vec.insert(5);
        assert_eq!(sorted_vec.size, 1);
        assert_eq!(sorted_vec.at(0), Some(&5));
    }

    #[test]
    fn sorted_vec_insert_triggers_split() {
        let config = BucketConfiguration::new(MaxBucketCapacity::new(1), 5);
        let mut sorted_vec: SortedVec<i32> = SortedVec::new(config);
        sorted_vec.insert(5);
        sorted_vec.insert(3);
        println!("{:?}", sorted_vec);

        assert_eq!(sorted_vec.buckets.len(), 2);
        assert_eq!(sorted_vec.size, 2);
        assert_eq!(sorted_vec.at(0), Some(&3));
        assert_eq!(sorted_vec.at(1), Some(&5));
    }

    #[test]
    fn sorted_vec_at_out_of_bounds() {
        let config = BucketConfiguration::new(MaxBucketCapacity::new(10), 5);
        let sorted_vec: SortedVec<i32> = SortedVec::new(config);
        assert_eq!(sorted_vec.at(0), None);
    }

    #[test]
    fn sorted_vec_remove_single_element() {
        let config = BucketConfiguration::new(MaxBucketCapacity::new(10), 5);
        let mut sorted_vec: SortedVec<i32> = SortedVec::new(config);
        sorted_vec.insert(5);
        sorted_vec.remove(&5);
        assert_eq!(sorted_vec.size, 0);
        assert_eq!(sorted_vec.at(0), None);
    }

    #[test]
    fn sorted_vec_remove_multiple_elements() {
        let config = BucketConfiguration::new(MaxBucketCapacity::new(10), 5);
        let mut sorted_vec: SortedVec<i32> = SortedVec::new(config);
        sorted_vec.insert(5);
        sorted_vec.insert(3);
        sorted_vec.insert(8);
        sorted_vec.remove(&3);
        sorted_vec.remove(&8);
        assert_eq!(sorted_vec.size, 1);
        assert_eq!(sorted_vec.at(0), Some(&5));
    }
}