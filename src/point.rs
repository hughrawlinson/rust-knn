use rand::thread_rng;
use rand::Rng;
use serde::Serialize;

use crate::{Distance, NewRandom};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
  x: f64,
  y: f64,
  z: f64,
}

impl Point {
  fn new(x: f64, y: f64, z: f64) -> Point {
    Point { x, y, z }
  }
}

impl NewRandom for Point {
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
