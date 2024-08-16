use std::cmp::Ordering;
use sorted_vec::sorted_vec::{BucketConfiguration, MaxBucketCapacity, SortedVec};

#[derive(Debug)]
struct Flight {
    id: u64,
    origin: String,
    destination: String,
    price: f64,
}

impl Flight {
    fn new(id: u64, origin: String, destination: String, price: f64) -> Self {
        Flight {
            id,
            origin,
            destination,
            price,
        }
    }
}

impl Eq for Flight {}

impl Ord for Flight {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.partial_cmp(&other.price).unwrap()
    }
}

impl PartialEq<Self> for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl PartialOrd for Flight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.price.partial_cmp(&other.price)
    }
}

fn main() {
    let mut flights = vec![
        Flight::new(5, "LAX".to_string(), "JFK".to_string(), 500.0),
        Flight::new(2, "JFK".to_string(), "LAX".to_string(), 200.0),
        Flight::new(3, "LAX".to_string(), "JFK".to_string(), 300.0),
        Flight::new(1, "LAX".to_string(), "JFK".to_string(), 100.0),
        Flight::new(4, "JFK".to_string(), "LAX".to_string(), 400.0),
    ];

    let sorted_vec = SortedVec::from_vec(flights, BucketConfiguration::with_max_bucket_capacity(MaxBucketCapacity::new(2)));

    println!("{:?}", sorted_vec.first());
    println!("{:?}", sorted_vec.last());
}
