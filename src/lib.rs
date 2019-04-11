mod common;
mod consts;
mod converters;
mod err;
mod hsl;
mod normalize;
pub mod ratio_converters;
mod rgb;

pub use common::alpha::ColorAlpha;
pub use common::approx::{ApproxEq, DEFAULT_APPROX_EQ_PRECISION};
pub use err::ParseError;
pub use hsl::Hsl;
pub use rgb::{GrayScaleMethod, Rgb};

// pub type ColorArr = [f64; 3];
// pub type ColorArrA = [f64; 4];

pub type ColorTuple = (f64, f64, f64);
pub type ColorTupleA = (f64, f64, f64, f64);

pub enum SaturationInSpace {
  Hsl(f64),
  Hsv(f64),
}
