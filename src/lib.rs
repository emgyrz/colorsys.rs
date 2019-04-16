
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

/// Use to transfer nad collect color values.
/// May be for example `($red,$green,$blue)` or `($hue,$saturation,$value)`
pub type ColorTuple = (f64, f64, f64);

/// For example `($hue,$saturation,$lightness,$alpha)`
pub type ColorTupleA = (f64, f64, f64, f64);


pub enum SaturationInSpace {
  Hsl(f64),
  Hsv(f64),
}

/// Methods to work with alpha channel in color.
pub trait ColorAlpha {
  /// Returns alpha channel. If it not setted will returns 1.0
  fn get_alpha(&self) -> f64;

  /// Sets alpha channel
  /// ```
  /// use colorsys::{Hsl,ColorAlpha};
  /// let mut hsl = Hsl::default(); // Hsl { a: None, .. }
  /// hsl.set_alpha(0.45); // Hsl { a: 0.45, .. }
  /// hsl.set_alpha(123.015); // Hsl { a: 1.0, .. }
  /// hsl.set_alpha(-123.3); // Hsl { a: 0.0, .. }
  /// ```
  fn set_alpha(&mut self, val: f64);

  /// Increase/decrease color alpha channel with specified value. Value can be negative.
  /// # Example
  /// ```
  /// use colorsys::{Hsl,ColorAlpha};
  /// let mut hsl = Hsl::default(); // Hsl { a: None, .. }
  /// hsl.opacify(-0.3); // Hsl { a: 0.7, .. }
  /// hsl.opacify(0.015); // Hsl { a: 0.715, .. }
  /// ```
  fn opacify(&mut self, val: f64);
}



/// A collection of methods to some special modification of color.
/// Some methods (like saturate, lighten, etc.) requires (inside implementation)
/// converting to another color space and converting back.
pub trait ColorTransform {
  /// Makes color lighter or (if amt is negative) darker. Amt is percent - `0..100`
  fn lighten(&mut self, amt: f64);

  /// Saturate/desaturate color. Value is percent - `0..100`.
  /// You need specify in what color space you want to increase/decrease saturation.
  fn saturate(&mut self, sat: SaturationInSpace);

  /// increase/decrease color tone. Value is degree - `0..360`.
  fn adjust_hue(&mut self, hue: f64);

  /// Brings color to a shade of gray. For more specific grayscale methods see `Rgb.grayscale`
  fn grayscale_simple(&mut self);

  /// Just inverts color
  fn invert(&mut self);

}
