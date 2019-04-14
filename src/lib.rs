mod common;
mod consts;
mod converters;
mod err;
mod hsl;
mod normalize;
mod rgb;

pub mod prelude;
pub mod ratio_converters;
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

pub trait ColorAlpha {
  fn get_alpha(&self) -> f64;
  fn set_alpha(&mut self, val: f64);
  fn opacify(&mut self, val: f64);
}

pub trait ColorTransform {
  fn lighten(&mut self, amt: f64);
  fn saturate(&mut self, sat: SaturationInSpace);
  fn adjust_hue(&mut self, hue: f64);
  fn grayscale_simple(&mut self);
  fn invert(&mut self);
}
