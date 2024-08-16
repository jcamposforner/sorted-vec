#[clippy::all]
mod bucket;
pub mod sorted_vec;

#[derive(Debug, PartialEq)]
pub enum AddResult {
    Added(usize),
    Duplicated(usize),
}