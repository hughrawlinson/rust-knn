use clap::{ArgEnum, Parser};
use serde::Serialize;

use nearest_neighbor::*;

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
  classification: String,
  query_point: T,
  nearest_neighbors: Vec<NeighborWithDistance>,
}

fn run<T: NewRandom + Distance>(args: Config) -> KnnResult<T> {
  let query_point = T::new_random();
  let dataset = create_dataset(args.dataset_size);

  let results = k_nearest_neighbors(args.k, &query_point, &dataset);
  let classification = classify(&results);

  KnnResult {
    classification,
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
