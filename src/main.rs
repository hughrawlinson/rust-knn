use clap::{ArgEnum, Parser};
use serde::Serialize;

mod datum;
mod point;
mod point2d;

use crate::datum::Datum;
use crate::point::Point;
use crate::point2d::Point2d;

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

#[derive(ArgEnum, Clone, Debug)]
enum PointType {
  TwoDimensional,
  ThreeDimensional,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Config {
  #[clap(short, long, default_value = "10")]
  k: usize,
  #[clap(short, long, default_value = "1000")]
  dataset_size: i32,
  #[clap(short, long, default_value = "two-dimensional", arg_enum)]
  point_type: PointType,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct NeighborWithDistance {
  neighbor_id: usize,
  distance: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct KnnResult<T> {
  query_point: T,
  nearest_neighbors: Vec<NeighborWithDistance>,
}

fn run<T: NewRandom + Distance>(args: Config) -> KnnResult<T> {
  let query_point = T::new_random();
  let dataset = create_dataset(args.dataset_size);

  let results = k_nearest_neighbors(args.k, &query_point, &dataset);

  KnnResult {
    query_point,
    nearest_neighbors: results
      .into_iter()
      .map(|(datum, distance)| NeighborWithDistance {
        neighbor_id: datum.id,
        distance,
      })
      .collect(),
  }
}

fn main() {
  let args = Config::parse();
  match args.point_type {
    PointType::TwoDimensional => {
      let knn_result: KnnResult<Point2d> = run(args);
      println!("{}", serde_json::to_string(&knn_result).unwrap());
    }
    PointType::ThreeDimensional => {
      let knn_result: KnnResult<Point> = run(args);
      println!("{}", serde_json::to_string(&knn_result).unwrap());
    }
  };
}
