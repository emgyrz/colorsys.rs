use crate::common::f64_abs;
use crate::units::GetColorUnits;

/// Default precision used for color comparison.
/// It is `0.000_000_001`
pub static DEFAULT_APPROX_EQ_PRECISION: f64 = 1e-9;

/// Methods to compare two colors
pub trait ApproxEq<T> {
  fn approx_eq(&self, other: &T) -> bool;
  fn approx_eq_clarify(&self, other: &T, precision: f64) -> bool;
}

pub fn approx(x: f64, y: f64, precision: f64) -> bool {
  f64_abs(x - y) < precision
}

impl<T> ApproxEq<T> for T where T: GetColorUnits {
  fn approx_eq(&self, other: &T) -> bool {
    self.get_units().approx_eq(other.get_units())
  }
  fn approx_eq_clarify(&self, other: &T, precision: f64) -> bool {
    self.get_units().approx_eq_clarify(other.get_units(), precision)
  }
}
