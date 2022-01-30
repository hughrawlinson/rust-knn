use clap::Parser;
use serde::Serialize;

mod datum;
mod point;

use crate::datum::Datum;
use crate::point::Point;

pub trait Distance {
  fn distance(&self, other: &Self) -> f64;
}

pub trait NewRandom {
  fn new_random() -> Self;
}

fn create_dataset<T: Distance + NewRandom>(number: i32) -> Vec<Datum<T>> {
  let mut dataset = Vec::new();
  for _i in 0..number {
    dataset.push(Datum::new_random());
  }
  dataset
}

fn k_nearest_neighbors<'a, T: Distance>(
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

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Config {
  #[clap(short, long, default_value = "10")]
  k: usize,
  #[clap(short, long, default_value = "1000")]
  dataset_size: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NeighborWithDistance {
  neighbor_id: i32,
  distance: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct KnnResult {
  query_point: Point,
  nearest_neighbors: Vec<NeighborWithDistance>,
}

fn main() {
  let args = Config::parse();

  let query_point = Point::new_random();

  let dataset = create_dataset(args.dataset_size);

  let results = k_nearest_neighbors(args.k, &query_point, &dataset);

  let knn_result = KnnResult {
    query_point,
    nearest_neighbors: results
      .into_iter()
      .map(|(datum, distance)| NeighborWithDistance {
        neighbor_id: datum.id,
        distance,
      })
      .collect(),
  };

  println!("{}", serde_json::to_string(&knn_result).unwrap());
}
