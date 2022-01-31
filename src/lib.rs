use std::collections::HashMap;

mod datum;
mod point;
mod point2d;

pub use datum::Datum;
pub use point::Point;
pub use point2d::Point2d;

pub trait Distance {
  fn distance(&self, other: &Self) -> f64;
}

pub trait NewRandom {
  fn new_random() -> Self;
}

pub fn create_dataset<T: Distance + NewRandom>(number: i32) -> Vec<Datum<T>> {
  let mut dataset = Vec::new();
  for _i in 0..number {
    dataset.push(Datum::new_random());
  }
  dataset
}

pub fn k_nearest_neighbors<'a, T: Distance>(
  k: usize,
  query_point: &T,
  dataset: &'a Vec<Datum<T>>,
) -> Vec<(&'a Datum<T>, f64)> {
  let mut dataset_with_distances: Vec<(&Datum<T>, f64)> = dataset
    .into_iter()
    .map(|datum| (datum, datum.distance_from_point(&query_point)))
    .collect();

  dataset_with_distances
    .sort_by(|(_, distance_a), (_, distance_b)| distance_a.partial_cmp(distance_b).unwrap());

  dataset_with_distances.into_iter().take(k).collect()
}

pub fn classify<T: Distance>(results: &Vec<(&Datum<T>, f64)>) -> String {
  let histogram: HashMap<String, i64> = HashMap::new();
  results
    .into_iter()
    .fold(histogram, |mut acc, (datum, _)| {
      acc.insert(
        datum.class.clone(),
        acc.get(&datum.class).map(|count| count + 1).unwrap_or(1),
      );
      acc
    })
    .into_iter()
    .max_by_key(|&(_, count)| count)
    .unwrap()
    .0
}
