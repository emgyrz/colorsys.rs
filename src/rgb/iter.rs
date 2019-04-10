use super::Rgb;
use crate::ColorTuple;

pub struct RgbIter {
  ind: usize,
  vals: [Option<f64>; 4],
}

impl RgbIter {
  pub fn new(t: ColorTuple, a: Option<f64>) -> RgbIter {
    RgbIter { ind: 0, vals: [Some(t.0), Some(t.1), Some(t.2), a] }
  }
}

impl std::iter::Iterator for RgbIter {
  type Item = f64;
  fn next(&mut self) -> Option<Self::Item> {
    match self.ind {
      0...3 => {
        let val = self.vals[self.ind];
        self.ind += 1;
        val
      }
      _ => None,
    }
  }
}

impl<'a> std::iter::IntoIterator for &'a Rgb {
  type Item = f64;
  type IntoIter = RgbIter;
  fn into_iter(self) -> RgbIter {
    self.iter()
  }
}

impl std::iter::IntoIterator for Rgb {
  type Item = f64;
  type IntoIter = RgbIter;
  fn into_iter(self) -> RgbIter {
    self.iter()
  }
}
