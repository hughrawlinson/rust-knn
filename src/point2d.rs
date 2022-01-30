use rand::thread_rng;
use rand::Rng;
use serde::Serialize;

use crate::{Distance, NewRandom};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Point2d {
  x: f64,
  y: f64,
}

impl Point2d {
  fn new(x: f64, y: f64) -> Point2d {
    Point2d { x, y }
  }
}

impl NewRandom for Point2d {
  fn new_random() -> Point2d {
    let mut rng = thread_rng();
    Point2d::new(rng.gen(), rng.gen())
  }
}

impl Distance for Point2d {
  fn distance(&self, b: &Point2d) -> f64 {
    let dx = self.x - b.x;
    let dy = self.y - b.y;
    (dx * dx + dy * dy).sqrt()
  }
}
