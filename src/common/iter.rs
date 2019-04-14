use crate::ColorTuple;

pub struct ColorIter {
  ind: usize,
  vals: [Option<f64>; 4],
}

impl ColorIter {
  pub fn from_tuple_w_alpha(t: ColorTuple, a: Option<f64>) -> ColorIter {
    ColorIter { ind: 0, vals: [Some(t.0), Some(t.1), Some(t.2), a] }
  }
}

impl std::iter::Iterator for ColorIter {
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
