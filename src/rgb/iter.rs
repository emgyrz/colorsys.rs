use super::Rgb;
use crate::ColorTuple;

pub struct RgbIter {
  ind: usize,
  vals: [Option<f32>; 4],
}

impl RgbIter {
  pub fn new(t: ColorTuple, a: Option<f32>) -> RgbIter {
    RgbIter { ind: 0, vals: [Some(t.0), Some(t.1), Some(t.2), a] }
  }
}

impl std::iter::Iterator for RgbIter {
  type Item = f32;
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
  type Item = f32;
  type IntoIter = RgbIter;
  fn into_iter(self) -> RgbIter {
    self.iter()
  }
}

impl std::iter::IntoIterator for Rgb {
  type Item = f32;
  type IntoIter = RgbIter;
  fn into_iter(self) -> RgbIter {
    self.iter()
  }
}
