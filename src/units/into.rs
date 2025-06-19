use crate::units::{Units};


impl Into<[f64; 3]> for &Units {
  fn into(self) -> [f64; 3] {
    [self[0], self[1], self[2]]
  }
}

impl Into<[f64; 3]> for Units {
  fn into(self) -> [f64; 3] {
    [self[0], self[1], self[2]]
  }
}

impl Into<[f64; 4]> for &Units {
  fn into(self) -> [f64; 4] {
    [self[0], self[1], self[2], self[3]]
  }
}

impl Into<[f64; 4]> for Units {
  fn into(self) -> [f64; 4] {
    [self[0], self[1], self[2], self[3]]
  }
}

