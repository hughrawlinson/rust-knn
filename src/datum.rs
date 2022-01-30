use crate::{Distance, NewRandom};
use rand::thread_rng;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Datum<T: Distance> {
  point: T,
  pub id: usize,
}

impl<T: Distance> Datum<T> {
  fn new(point: T, id: usize) -> Datum<T> {
    Datum { point, id }
  }

  pub fn distance_from_point(&self, point: &T) -> f64 {
    self.point.distance(point)
  }
}

impl<T: Distance + NewRandom> NewRandom for Datum<T> {
  fn new_random() -> Datum<T> {
    let point = T::new_random();
    let id = thread_rng().gen();
    Datum::new(point, id)
  }
}

impl<T: Distance> Distance for Datum<T> {
  fn distance(&self, datum: &Datum<T>) -> f64 {
    self.point.distance(&datum.point)
  }
}
