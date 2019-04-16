use crate::ColorTuple;

/// Default precision used for color comparsion.
/// It is `0.000_000_001`
pub static DEFAULT_APPROX_EQ_PRECISION: f64 = 0.000_000_001;

/// Methods to compare two colors
pub trait ApproxEq<T> {
  fn approx_eq(&self, other: &T) -> bool;
  fn approx_eq_clarify(&self, other: &T, precision: f64) -> bool;
}

pub fn approx(x: f64, y: f64, precision: f64) -> bool {
  (x - y).abs() < precision
}
pub fn approx_def(x: f64, y: f64) -> bool {
  approx(x, y, DEFAULT_APPROX_EQ_PRECISION)
}

pub fn approx_tuple(x: &ColorTuple, y: &ColorTuple, precision: f64) -> bool {
  approx(x.0, y.0, precision) && approx(x.1, y.1, precision) && approx(x.2, y.2, precision)
}

pub fn approx_tuple_def(x: &ColorTuple, y: &ColorTuple) -> bool {
  approx_def(x.0, y.0) && approx_def(x.1, y.1) && approx_def(x.2, y.2)
}
