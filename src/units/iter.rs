use super::Units;

#[doc(hidden)]
pub struct ColorUnitsIter {
  ind: usize,
  values: [f64; 5],
  len: usize,
}

impl ColorUnitsIter {
  pub(crate) fn from_units(units: &Units) -> ColorUnitsIter {
    let mut values = [0.0; 5];
    for i in 0..units.len {
      values[i] = units.list[i].value;
    }

    let mut len = units.len;
    if let Some(a) = units.alpha.get() {
      values[len] = a;
      len += 1;
    }

    ColorUnitsIter { ind: 0, values, len }
  }
}


impl core::iter::Iterator for ColorUnitsIter {
  type Item = f64;
  fn next(&mut self) -> Option<Self::Item> {
    if self.ind == self.len { return None; }
    let v = self.values[self.ind];
    self.ind += 1;
    Some(v)
  }
}


#[cfg(test)]
mod test {
  use crate::Rgb;
  #[cfg(not(feature = "std"))]
  use alloc::vec::Vec;


  #[test]
  fn color_iter_collect_test() {
    let rgb = Rgb::new(1.0, 2.0, 3.0, None);
    let v: Vec<f64> = rgb.iter().collect();
    assert_eq!(v, vec![1.0, 2.0, 3.0]);

    let rgba = Rgb::new(1.0, 2.0, 3.0, Some(0.5));
    let v: Vec<f64> = rgba.iter().collect();
    assert_eq!(v, vec![1.0, 2.0, 3.0, 0.5]);
  }

  #[test]
  fn color_iter_test() {
    let rgb = Rgb::new(1.0, 2.0, 3.0, None);
    let mut n = 1.0;

    for c in &rgb {
      assert!(n.eq(&c));
      n += 1.0;
    }
  }
}
