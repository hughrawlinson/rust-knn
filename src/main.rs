use rand::thread_rng;
use rand::Rng;

#[derive(Clone, Debug)]
struct Point {
  x: f64,
  y: f64,
  z: f64,
}

trait Distance {
  fn distance(&self, other: &Self) -> f64;
}

impl Point {
  fn new(x: f64, y: f64, z: f64) -> Point {
    Point { x, y, z }
  }

  fn new_random() -> Point {
    let mut rng = thread_rng();
    Point::new(rng.gen(), rng.gen(), rng.gen())
  }
}

impl Distance for Point {
  fn distance(&self, b: &Point) -> f64 {
    let dx = self.x - b.x;
    let dy = self.y - b.y;
    let dz = self.z - b.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
  }
}

#[derive(Clone, Debug)]
struct Datum {
  point: Point,
  id: i32,
}

impl Datum {
  fn new(point: Point, id: i32) -> Datum {
    Datum { point, id }
  }

  // fn new_random() -> Datum {
  //   let point = Point::new_random();
  //   let id = thread_rng().gen();
  //   Datum::new(point, id)
  // }

  fn new_random(id: i32) -> Datum {
    let point = Point::new_random();
    Datum::new(point, id)
  }

  fn distance_from_point(&self, point: &Point) -> f64 {
    self.point.distance(point)
  }
}

impl Distance for Datum {
  fn distance(&self, datum: &Datum) -> f64 {
    self.point.distance(&datum.point)
  }
}

fn create_dataset(number: i32) -> Vec<Datum> {
  let mut dataset = Vec::new();
  for i in 0..number {
    dataset.push(Datum::new_random(i));
  }
  dataset
}

fn k_nearest_neighbors(k: usize, query_point: Point, dataset: &Vec<Datum>) -> Vec<(&Datum, f64)> {
  let mut dataset_with_distances: Vec<(&Datum, f64)> = dataset
    .iter()
    .map(|datum| (datum, datum.distance_from_point(&query_point)))
    .collect();

  dataset_with_distances
    .sort_by(|(_, distance_a), (_, distance_b)| distance_a.partial_cmp(distance_b).unwrap());

  dataset_with_distances.into_iter().take(k).collect()
}

fn main() {
  let dataset = create_dataset(100000);

  let query_point = Point::new_random();
  println!("query_point: {:?}", query_point);

  let results = k_nearest_neighbors(3, query_point, &dataset);
  for (datum, distance) in results {
    println!("{:?} {:?}", datum.id, distance);
  }
}
