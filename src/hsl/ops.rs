use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{ApproxEq, DEFAULT_APPROX_EQ_PRECISION, Rgb};

use super::Hsl;

ops_def!(Hsl);

impl ApproxEq<Rgb> for Hsl {
  fn approx_eq(&self, rgb: &Rgb) -> bool {
    self.approx_eq_clarify(rgb, DEFAULT_APPROX_EQ_PRECISION)
  }
  fn approx_eq_clarify(&self, rgb: &Rgb, precision: f64) -> bool {
    let hsl_from_rgb: Hsl = rgb.into();
    self.units.approx_eq_clarify(&hsl_from_rgb.units, precision)
  }
}

#[test]
fn hsl_add() {}

#[test]
fn hsl_eq() {}
