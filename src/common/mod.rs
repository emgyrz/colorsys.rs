pub use alpha::ColorAlpha;
pub use hsv_hsl_from_str::hsl_hsv_from_str;
pub use tuple_to_string::tuple_to_string;

pub use crate::units::iter::ColorUnitsIter;

mod alpha;
mod hsv_hsl_from_str;
mod tuple_to_string;

pub mod approx;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Hs {
  #[allow(dead_code)]
  Hsv,

  Hsl,
}

#[cfg(feature = "std")]
pub(crate) fn f64_abs(n: f64) -> f64 {
  n.abs()
}

#[cfg(not(feature = "std"))]
pub(crate) fn f64_abs(n: f64) -> f64 {
  if n < 0.0 { -n } else { n }
}

#[cfg(feature = "std")]
pub(crate) fn f64_round(n: f64) -> f64 {
  n.round()
}

#[cfg(not(feature = "std"))]
pub(crate) fn f64_round(n: f64) -> f64 {
  let f = if n == 0.0 { 0.0 } else { n % 1.0 };

  if f.is_nan() || f == 0.0 {
    n
  } else if n > 0.0 {
    if f < 0.5 { n - f } else { n - f + 1.0 }
  } else if -f < 0.5 {
    n - f
  } else {
    n - f - 1.0
  }
}

#[cfg(test)]
mod f64_round_test {
  use crate::common::f64_round;

  fn _test_data() -> [(f64, f64); 5] {
    [(0.3, 0.0), (0.7134, 1.0), (-1.13, -1.0), (100.45, 100.0), (55.5, 56.0)]
  }

  fn _run() {
    _test_data().into_iter().for_each(|(n, rounded)| {
      assert_eq!(f64_round(n), rounded);
    });
  }

  #[test]
  #[cfg(feature = "std")]
  fn f64_round_std_test() {
    _run()
  }
}
