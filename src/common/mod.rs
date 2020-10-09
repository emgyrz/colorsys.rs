mod hsv_hsl_from_str;
mod iter;
mod tuple_to_string;

pub mod approx;
pub mod ops;

pub use hsv_hsl_from_str::hsl_hsv_from_str;
pub use iter::ColorIter;
pub use tuple_to_string::tuple_to_string;

pub enum Hs {
  Hsv,
  Hsl,
}
