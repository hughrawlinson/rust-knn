use crate::{Distance, NewRandom};
use rand::thread_rng;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Datum<T: Distance> {
  pub class: String,
  point: T,
  pub id: i32,
}

impl<T: Distance> Datum<T> {
  fn new(point: T, id: i32, class: String) -> Datum<T> {
    Datum { point, id, class }
  }

  pub fn distance_from_point(&self, point: &T) -> f64 {
    self.point.distance(point)
  }
}

impl<T: Distance + NewRandom> NewRandom for Datum<T> {
  fn new_random() -> Datum<T> {
    let point = T::new_random();
    let id = thread_rng().gen();
    let class = if thread_rng().gen::<f32>() > 0.5 {
      "class_1".to_string()
    } else {
      "class_2".to_string()
    };
    Datum::new(point, id, class)
  }
}

impl<T: Distance> Distance for Datum<T> {
  fn distance(&self, datum: &Datum<T>) -> f64 {
    self.point.distance(&datum.point)
  }
}
