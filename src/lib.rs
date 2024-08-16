mod bucket;
pub mod sorted_vec;
mod sorted_vec_iter;

pub mod iter {
    pub use crate::sorted_vec_iter::SortedVecIter;
}

#[derive(Debug, PartialEq)]
pub enum AddResult {
    Added(usize),
    Duplicated(usize),
}
