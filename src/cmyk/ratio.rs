use crate::units::Units;

#[derive(Debug, PartialEq, Clone)]
pub struct CmykRatio {
  pub(crate) units: Units,
}

impl CmykRatio {
  pub fn new(c: f64, m: f64, y: f64, k: f64, a: f64) -> Self {
    let mut u = Units::new_ratios(&[c, m, y, k]);
    u.alpha.set(a);
    CmykRatio::from_units(u)
  }

  pub fn cyan(&self) -> f64 { self.units[0] }
  pub fn magenta(&self) -> f64 { self.units[1] }
  pub fn yellow(&self) -> f64 { self.units[2] }
  pub fn key(&self) -> f64 { self.units[3] }

  pub fn set_cyan(&mut self, c: f64) { self.units.list[0].set(c); }
  pub fn set_magenta(&mut self, m: f64) { self.units.list[1].set(m); }
  pub fn set_yellow(&mut self, y: f64) { self.units.list[2].set(y); }
  pub fn set_key(&mut self, k: f64) { self.units.list[3].set(k); }

  pub(crate) fn from_units(u: Units) -> Self {
    CmykRatio { units: u }
  }
}


impl AsRef<CmykRatio> for CmykRatio {
  fn as_ref(&self) -> &CmykRatio { self }
}

impl Default for CmykRatio {
  fn default() -> CmykRatio {
    CmykRatio::new(0.0, 0.0, 0.0, 1.0, 1.0)
  }
}

impl From<[f64; 4]> for CmykRatio {
  fn from(a: [f64; 4]) -> Self { CmykRatio::new(a[0], a[1], a[2], a[3], 1.0) }
}

impl<'a> From<&'a [f64; 4]> for CmykRatio {
  fn from(a: &[f64; 4]) -> Self { CmykRatio::new(a[0], a[1], a[2], a[3], 1.0) }
}

impl Into<[f64; 4]> for CmykRatio {
  fn into(self: CmykRatio) -> [f64; 4] { self.units.into() }
}

impl<'a> Into<[f64; 4]> for &'a CmykRatio {
  fn into(self) -> [f64; 4] { self.units.clone().into() }
}

#[cfg(test)]
mod test {
  use crate::{Cmyk, Rgb};
  use crate::converters::cmyk_to_rgb;

  #[test]
  fn cmyk_to_rbg_test() {
    let cmyk = Cmyk::new(30.0, 30.0, 30.0, 30.0, None);
    let _rgb: Rgb = cmyk_to_rgb(&cmyk);
  }
}
