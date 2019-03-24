mod common;
mod consts;
mod err;
mod hsl;
mod normalize;
pub mod ratio_converters;
mod rgb;

pub use err::ParseError;
pub use hsl::Hsl;
pub use rgb::{GrayScaleMethod, Rgb};

// pub type ColorArr = [f32; 3];
// pub type ColorArrA = [f32; 4];

pub type ColorTuple = (f32, f32, f32);
pub type ColorTupleA = (f32, f32, f32, f32);

pub enum SaturationInSpace {
  Hsl(f32),
  Hsv(f32),
}
