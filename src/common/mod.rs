pub use alpha::ColorAlpha;
pub use hsv_hsl_from_str::hsl_hsv_from_str;
pub use tuple_to_string::tuple_to_string;

pub use crate::units::iter::ColorUnitsIter;

mod hsv_hsl_from_str;
mod tuple_to_string;
mod alpha;
mod from_str;

pub mod approx;

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
  f64::from(n as i32)
}
