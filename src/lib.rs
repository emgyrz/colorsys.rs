//! A module for color conversion and mutation.
//!
//! For now you can work with four color representation options: Rgb, Rgba, Hsl, Hsla. Each of them has a variety of methods to modify and convert. See the Color trait they implement. There are also a couple of methods for hex string color.
//!
//! All values are given as f32 for more accurate calculations.
//!
//! ## What It Can Do
//!
//! ### getters & setters
//! ```
//! use colors_transform::{Rgb, Color};
//! let rgb = Rgb::from_tuple((57.3, 12.7, 53.0));
//! // where tuple is ($red, $green, $blue)
//!
//! let modified = rgb
//!   .set_red(245.0) // Rgb { r: 245.0, g: 152.0, b: 53.0 }
//!   .set_green(152.0) // Rgb { r: 245.0, g: 152.0, b: 53.0 }
//!   .set_hue(279.0); // Rgb { r: 177.80003, g: 53.00001, b: 245.0 }
//!
//! let saturation = modified.get_saturation(); // 63.71429
//! let blue = modified.get_blue(); // 53.00001
//!
//! ```
//!
//! ### conversion
//! ```ignore
//! let hex_color = Hsl::from_tuple((315.9, 99.7, 50.0))
//! // where tuple is ($hue, $saturation, $lightness)
//!   .to_rgb() // ~Rgb { r: 254.6, g: 0.38, b: 187.24 }
//!   .set_saturation(33.3) // ~Rgb { r: 169.9, g: 85.04, b: 147.45 }
//!   .to_hsla() // Hsla { h: 315.9, s: 33.3, l: 50.0 alpha: 1.0 }
//!   .set_alpha(0.47) // Hsla { h: 315.9, s: 99.7, l: 50.0 alpha: 0.47 }
//!   .to_rgb() // Rgb { r: 169.95749, g: 85.0425, b: 147.45502 }
//!   .to_css_hex_string(); // #aa5593
//! ```
//!
//! ### modification
//! ```ignore
//! let rgb = Rgb::from_tuple((245.0,152.0,53.0))
//!   .lighten(21.0) // Rgb { r: 250.05188, g: 204.03442, b: 155.04813 }
//!   .saturate( 3.9999 ); // Rgb { r: 252.14981, g: 204.1, b: 152.9502 }
//! // TODO: grayscale, invert and other
//! ```
//!
//! ### parsing from string & css string representation
//! ```ignore
//! let hsla: Hsla = "hsla(359,12%,71,0.3)".parse().unwrap();
//! // Hsla { h: 359.0, s: 12.0, l: 71.0 alpha: 0.3 }
//!
//! let rgb1 = "rgb(12,13,14)"
//!   .parse::<Rgb>()
//!   .unwrap()
//!   .adjust_color( RgbColor::Green, 139.7 );
//! // Rgb { r: 12.0, g: 152.7, b: 14.0 }
//!
//! let rgb2 = Rgb::from_hex_str("#fc0").unwrap();
//! // Rgb { r: 255.0, g: 204.0, b: 0.0 }
//!
//! let rgb_str = rgb1.to_css_string();
//! // rgb(12,153,14)
//!
//! let hsla_str = rgb2.to_hsla().to_css_string();
//! // "hsla(48,100%,50%,1)"
//! ```
//!
//! As you see it is completely chainable.
//!
//!
//! ## Color unit ranges
//! All color units is f32. Here are their ranges:
//!  - red - 0.0 .. 255.0
//!  - green - 0.0 .. 255.0
//!  - blue - 0.0 .. 255.0
//!  - hue - 0.0 .. 359.0
//!  - saturation - 0.0 .. 100.0
//!  - lightness - 0.0 .. 100.0
//!  - alpha - 0.0 .. 1.0
//!
//! If you specify a value that does not fit within these ranges, they are replaced with a minimum or maximum value.
//!
//!
//! <p style="margin: 50px 0">Enjoy using!</p>
//! <style>
//! h3 {border:none !important; margin: 30px 0px 0px 0px !important}
//! </style>

mod colors;
mod converters;
mod error;
mod from_str;
mod normalize;

#[cfg(test)]
mod tests;

pub use colors::{Hsl, Hsla, Rgb, Rgba};

pub use error::ParseError;

/// Tuple type just for data exchange. May be a:
/// - `($red,$green,$blue)` _`(0.0..255.0, 0.0..255.0, 0.0..255.0)`_
///
/// or
///
/// - `($hue, $saturation, $lightness)` _`(0.0..359.0, 0.0..100.0, 0.0..100.0)`_
/// # Example
/// ```
/// use colors_transform::{Rgb,Color,Hsl};
///
/// let rgb = Rgb::from_tuple((255.0,13.0,177.0));
/// assert_eq!(rgb.get_red(), 255.0);
///
/// let hsl: Hsl = "hsl(315,99,12)".parse().unwrap();
/// assert_eq!(hsl.get_saturation(), 99.0);
/// ```
pub type ColorTuple = (f32, f32, f32);

/// Like a `ColorTuple` but with fourth alpha value
pub type ColorTupleA = (f32, f32, f32, f32);

/// Common to all trait
pub trait Color {
  /// ColorTuple or ColorTupleA.
  ///

  type Tuple;
  /// Creates a black color
  /// # Example
  /// ```
  /// use colors_transform::{Rgb,Color};
  ///
  /// let black = Rgb::from_tuple((0.0,0.0,0.0));
  /// assert_eq!(black, Rgb::new());
  /// ```
  fn new() -> Self;

  /// Creates a color from tuple.
  /// # Example
  /// ```
  /// use colors_transform::{Rgba,Hsl,Color};
  ///
  /// let rgba = Rgba::from_tuple((10.0,11.0,12.0, 0.5));
  /// let hsl = Hsl::from_tuple((310.0,50.0,50.0));
  /// ```
  fn from_tuple(tuple: Self::Tuple) -> Self;

  /// Returns tuple representation of color
  /// # Example
  /// ```
  /// use colors_transform::{Hsla,Color};
  ///
  /// let hsla = Hsla::from_tuple((10.0,11.0,12.0, 0.5));
  /// assert_eq!((10.0,11.0,12.0, 0.5),hsla.as_tuple());
  /// ```
  fn as_tuple(&self) -> Self::Tuple;

  /// Returns red value of color (`0.0..255.00`)
  fn get_red(&self) -> f32;

  /// Returns green value of color (`0.0..255.00`)
  fn get_green(&self) -> f32;

  /// Returns blue value of color (`0.0..255.00`)
  fn get_blue(&self) -> f32;

  /// Sets red value of color (`0.0..255.00`). Returns Color
  fn set_red(&self, val: f32) -> Self;

  /// Sets green value of color (`0.0..255.00`). Returns Color
  fn set_green(&self, val: f32) -> Self;

  /// Sets blue value of color (`0.0..255.00`). Returns Color
  fn set_blue(&self, val: f32) -> Self;

  /// Returns hue value of color (`0.0..359.00`)
  fn get_hue(&self) -> f32;

  /// Returns saturation value of color (`0.0..100.00`)
  fn get_saturation(&self) -> f32;

  /// Returns lightness value of color (`0.0..100.00`)
  fn get_lightness(&self) -> f32;

  /// Sets hue value of color (`0.0..359.00`). Returns Color
  fn set_hue(&self, val: f32) -> Self;

  /// Sets saturation value of color (`0.0..100.00`). Returns Color
  fn set_saturation(&self, val: f32) -> Self;

  /// Sets lightness value of color (`0.0..100.00`). Returns Color
  fn set_lightness(&self, val: f32) -> Self;

  fn to_rgb(&self) -> Rgb;
  fn to_rgba(&self) -> Rgba;
  fn to_hsl(&self) -> Hsl;
  fn to_hsla(&self) -> Hsla;

  /// Returns css string
  /// # Example
  /// ```
  /// use colors_transform::{Hsl,Color};
  ///
  /// let hsl = Hsl::from_tuple((301.0,27.0,91.0));
  /// assert_eq!(hsl.to_css_string(), "hsl(301,27%,91%)");
  /// ```
  fn to_css_string(&self) -> String;

  fn adjust_hue(&self, amt: f32) -> Self;
  fn saturate(&self, amt: f32) -> Self;
  fn lighten(&self, amt: f32) -> Self;
  fn adjust_color(&self, col_name: RgbColor, val: f32) -> Self;
  fn grayscale(&self) -> Self;
}

/// Some methods for working with alpha channel for Rgba & Hsla
pub trait AlphaColor {
  fn get_alpha(&self) -> f32;
  fn set_alpha(&self, a: f32) -> Self;
  fn opacify(&self, o: f32) -> Self;
}

#[derive(Clone, Copy)]
pub enum RgbColor {
  Red,
  Green,
  Blue,
}
