//! All color units is f32. Here are their ranges:
//! - red - 0.0 .. 255.0
//! - green - 0.0 .. 255.0
//! - blue - 0.0 .. 255.0
//! - hue - 0.0 .. 360.0
//! - saturation - 0.0 .. 100.0
//! - lightness - 0.0 .. 100.0
//! - alpha - 0.0 .. 1.0
//!
//! If you specify a value that does not fit within these ranges, they are replaced with a minimum or maximum value.
mod converters;
mod error;
mod from_str;
mod normalize;

// mod colors;
// pub use colors::rgb::{Rgb,RgbColor};

#[cfg(test)]
mod tests;

use converters::{
  as_rounded_hsl_tuple, as_rounded_rgb_tuple, hsl_to_rgb, rgb_to_hex, rgb_to_hsl, round_ratio,
};
use normalize::{
  normalize_hsl, normalize_hue, normalize_percent, normalize_ratio, normalize_rgb,
  normalize_rgb_unit,
};

pub use error::ParseError;

pub type ColorTuple = (f32, f32, f32);
pub type ColorTupleA = (f32, f32, f32, f32);

/// Common to all trait
pub trait Color {
  type Tuple;
  fn new() -> Self;
  fn to_rgb(&self) -> Rgb;
  fn to_rgba(&self) -> Rgba;
  fn to_hsl(&self) -> Hsl;
  fn to_hsla(&self) -> Hsla;
  fn to_css_string(&self) -> String;
  fn from_tuple(tuple: Self::Tuple) -> Self;
  fn as_tuple(&self) -> Self::Tuple;
  fn adjust_hue(&self, amt: f32) -> Self;
  fn lighten(&self, amt: f32) -> Self;
  fn saturate(&self, amt: f32) -> Self;
  fn adjust_color(&self, col_name: RgbColor, val: f32) -> Self;
  fn get_unit(&self, unit: ColorUnit) -> f32;
  fn set_unit(&self, unit: ColorUnit, val: f32) -> Self;
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

pub enum ColorUnit {
  Red,
  Green,
  Blue,
  Hue,
  Saturation,
  Lightness,
}

//
//
// RGB
//
//
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb {
  r: f32,
  g: f32,
  b: f32,
}

impl Rgb {
  /// Try to parse string as hex color
  /// # Example
  /// ```
  /// use colors_transform::{Rgb,Color};
  ///
  /// assert_eq!(Rgb::from_hex_str("#e76B2c").unwrap(),Rgb::from_tuple((231.0,107.0,44.0)));
  /// assert_eq!(Rgb::from_hex_str("fc0").unwrap(),Rgb::from_tuple((255.0,204.0,0.0)));
  /// assert!(Rgb::from_hex_str("cyan").is_err());
  /// ```
  pub fn from_hex_str(s: &str) -> Result<Rgb, ParseError> {
    match from_str::hex(s) {
      Ok(rgb_tuple) => Ok(Rgb::from_tuple(rgb_tuple)),
      Err(err) => Err(err),
    }
  }
  /// Returns hexadecimal color string like in css. In lowercase with no reductions
  /// # Example
  /// ```
  /// use colors_transform::{Rgb,Color};
  ///
  /// let rgb1 = Rgb::from_tuple((231.0,107.0,44.0));
  /// assert_eq!(rgb1.to_css_hex_string(),"#e76b2c");
  ///
  /// let rgb2 = Rgb::from_hex_str("#0C7").unwrap();
  /// assert_eq!(rgb2.to_css_hex_string(),"#00cc77");
  /// ```
  pub fn to_css_hex_string(&self) -> String {
    let (r, g, b) = rgb_to_hex(&self.as_tuple());
    format!("#{}{}{}", r, g, b)
  }

  fn set_red(&self, val: f32) -> Rgb {
    Rgb { r: normalize_rgb_unit(val), g: self.g, b: self.b }
  }
  fn set_green(&self, val: f32) -> Rgb {
    Rgb { r: self.r, g: normalize_rgb_unit(val), b: self.b }
  }
  fn set_blue(&self, val: f32) -> Rgb {
    Rgb { r: self.r, g: self.g, b: normalize_rgb_unit(val) }
  }
}

impl std::str::FromStr for Rgb {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Rgb, ParseError> {
    match from_str::rgb(s) {
      Ok(rgb_tuple) => Ok(Rgb::from_tuple(rgb_tuple)),
      Err(err) => Err(err),
    }
  }
}

impl Color for Rgb {
  type Tuple = ColorTuple;

  fn new() -> Rgb {
    Rgb { r: 0.0, g: 0.0, b: 0.0 }
  }

  fn to_rgb(&self) -> Rgb {
    *self
  }
  fn to_rgba(&self) -> Rgba {
    let (r, g, b) = self.as_tuple();
    Rgba { rgb: Rgb::from_tuple((r, g, b)), alpha: 1.0 }
  }
  fn to_hsl(&self) -> Hsl {
    Hsl::from_tuple(rgb_to_hsl(&self.as_tuple()))
  }
  fn to_hsla(&self) -> Hsla {
    self.to_hsl().to_hsla()
  }
  /// Returns css string
  /// # Example
  /// ```
  /// use colors_transform::{Rgb,Color};
  ///
  /// assert_eq!(Rgb::from_tuple((225.0,101.7, 21.0)).to_css_string(), "rgb(225,102,21)");
  /// ```
  fn to_css_string(&self) -> String {
    let (r, g, b) = as_rounded_rgb_tuple(&self.as_tuple());
    format!("rgb({},{},{})", r, g, b)
  }

  fn from_tuple(t: ColorTuple) -> Rgb {
    let (r, g, b) = normalize_rgb(&t);
    Rgb { r, g, b }
  }
  fn as_tuple(&self) -> ColorTuple {
    (self.r, self.g, self.b)
  }

  fn lighten(&self, amt: f32) -> Rgb {
    self.to_hsl().lighten(amt).to_rgb()
  }
  fn saturate(&self, amt: f32) -> Rgb {
    self.to_hsl().saturate(amt).to_rgb()
  }
  fn adjust_hue(&self, amt: f32) -> Rgb {
    self.to_hsl().adjust_hue(amt).to_rgb()
  }
  fn adjust_color(&self, name: RgbColor, val: f32) -> Rgb {
    let (r, g, b) = self.as_tuple();
    match name {
      RgbColor::Red => self.set_red(r + val),
      RgbColor::Green => self.set_green(g + val),
      RgbColor::Blue => self.set_blue(b + val),
    }
  }

  fn get_unit(&self, unit: ColorUnit) -> f32 {
    match unit {
      ColorUnit::Red => self.r,
      ColorUnit::Green => self.g,
      ColorUnit::Blue => self.b,
      ColorUnit::Hue => self.to_hsl().h,
      ColorUnit::Saturation => self.to_hsl().s,
      ColorUnit::Lightness => self.to_hsl().l,
    }
  }
  fn set_unit(&self, unit: ColorUnit, val: f32) -> Rgb {
    match unit {
      ColorUnit::Red => self.set_red(val),
      ColorUnit::Green => self.set_green(val),
      ColorUnit::Blue => self.set_blue(val),
      ColorUnit::Hue => self.to_hsl().set_h(val).to_rgb(),
      ColorUnit::Saturation => self.to_hsl().set_s(val).to_rgb(),
      ColorUnit::Lightness => self.to_hsl().set_l(val).to_rgb(),
    }
  }
}

//
//
// RGBA
//
//
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
  rgb: Rgb,
  alpha: f32,
}

impl std::str::FromStr for Rgba {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Rgba, ParseError> {
    match from_str::rgba(s) {
      Ok(rgba_tuple) => Ok(Rgba::from_tuple(rgba_tuple)),
      Err(err) => Err(err),
    }
  }
}

impl AlphaColor for Rgba {
  fn get_alpha(&self) -> f32 {
    self.alpha
  }
  fn set_alpha(&self, a: f32) -> Rgba {
    let (r, g, b, _) = self.as_tuple();
    Rgba { rgb: Rgb::from_tuple((r, g, b)), alpha: normalize_ratio(a) }
  }
  fn opacify(&self, a: f32) -> Rgba {
    self.set_alpha(self.alpha + a)
  }
}

impl Color for Rgba {
  type Tuple = ColorTupleA;

  fn new() -> Rgba {
    Rgba { rgb: Rgb::new(), alpha: 1.0 }
  }
  fn to_rgb(&self) -> Rgb {
    self.rgb
  }
  fn to_rgba(&self) -> Rgba {
    *self
  }
  fn to_hsl(&self) -> Hsl {
    self.rgb.to_hsl()
  }
  fn to_hsla(&self) -> Hsla {
    Hsla { hsl: self.rgb.to_hsl(), alpha: self.alpha }
  }
  fn to_css_string(&self) -> String {
    let (r, g, b) = as_rounded_rgb_tuple(&self.rgb.as_tuple());
    format!("rgba({},{},{},{})", r, g, b, round_ratio(self.alpha))
  }

  fn from_tuple(t: ColorTupleA) -> Rgba {
    let (r, g, b, a) = t;
    Rgba { rgb: Rgb::from_tuple((r, g, b)), alpha: normalize_ratio(a) }
  }
  fn as_tuple(&self) -> ColorTupleA {
    let (r, g, b) = self.rgb.as_tuple();
    (r, g, b, self.alpha)
  }
  fn lighten(&self, amt: f32) -> Rgba {
    Rgba { rgb: self.rgb.lighten(amt), alpha: self.alpha }
  }
  fn saturate(&self, amt: f32) -> Rgba {
    Rgba { rgb: self.rgb.saturate(amt), alpha: self.alpha }
  }
  fn adjust_hue(&self, amt: f32) -> Rgba {
    Rgba { rgb: self.rgb.adjust_hue(amt), alpha: self.alpha }
  }
  fn adjust_color(&self, name: RgbColor, val: f32) -> Rgba {
    Rgba { rgb: self.rgb.adjust_color(name, val), alpha: self.alpha }
  }
  fn get_unit(&self, unit: ColorUnit) -> f32 {
    self.rgb.get_unit(unit)
  }
  fn set_unit(&self, unit: ColorUnit, val: f32) -> Rgba {
    Rgba { rgb: self.rgb.set_unit(unit, val), alpha: self.alpha }
  }
}

//
//
// HSL
//
//
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsl {
  h: f32,
  s: f32,
  l: f32,
}

impl Hsl {
  fn set_h(&self, val: f32) -> Hsl {
    Hsl { h: normalize_hue(val), s: self.s, l: self.l }
  }
  fn set_s(&self, val: f32) -> Hsl {
    Hsl { h: self.h, s: normalize_percent(val), l: self.l }
  }
  fn set_l(&self, val: f32) -> Hsl {
    Hsl { h: self.h, s: self.s, l: normalize_percent(val) }
  }
}

impl std::str::FromStr for Hsl {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Hsl, ParseError> {
    match from_str::hsl(s) {
      Ok(hsl_tuple) => Ok(Hsl::from_tuple(hsl_tuple)),
      Err(err) => Err(err),
    }
  }
}

impl Color for Hsl {
  type Tuple = ColorTuple;
  fn new() -> Hsl {
    Hsl { h: 0.0, s: 0.0, l: 0.0 }
  }
  fn to_rgb(&self) -> Rgb {
    Rgb::from_tuple(hsl_to_rgb(&self.as_tuple()))
  }
  fn to_rgba(&self) -> Rgba {
    self.to_rgb().to_rgba()
  }
  fn to_hsl(&self) -> Hsl {
    *self
  }
  fn to_hsla(&self) -> Hsla {
    Hsla { hsl: *self, alpha: 1.0 }
  }
  fn to_css_string(&self) -> String {
    let (h, s, l) = as_rounded_hsl_tuple(&self.as_tuple());
    format!("hsl({},{}%,{}%)", h, s, l)
  }
  fn from_tuple(t: ColorTuple) -> Hsl {
    let (h, s, l) = normalize_hsl(&t);
    Hsl { h, s, l }
  }
  fn as_tuple(&self) -> ColorTuple {
    (self.h, self.s, self.l)
  }
  fn lighten(&self, val: f32) -> Hsl {
    self.set_l(self.l + val)
  }
  fn saturate(&self, val: f32) -> Hsl {
    self.set_s(self.s + val)
  }
  fn adjust_hue(&self, hue: f32) -> Hsl {
    self.set_h(self.h + hue)
  }
  fn adjust_color(&self, name: RgbColor, val: f32) -> Hsl {
    self.to_rgb().adjust_color(name, val).to_hsl()
  }

  fn get_unit(&self, unit: ColorUnit) -> f32 {
    match unit {
      ColorUnit::Red => self.to_rgb().r,
      ColorUnit::Green => self.to_rgb().g,
      ColorUnit::Blue => self.to_rgb().b,
      ColorUnit::Hue => self.h,
      ColorUnit::Saturation => self.s,
      ColorUnit::Lightness => self.l,
    }
  }
  fn set_unit(&self, unit: ColorUnit, val: f32) -> Hsl {
    match unit {
      ColorUnit::Red => self.to_rgb().set_red(val).to_hsl(),
      ColorUnit::Green => self.to_rgb().set_green(val).to_hsl(),
      ColorUnit::Blue => self.to_rgb().set_blue(val).to_hsl(),
      ColorUnit::Hue => self.set_h(val),
      ColorUnit::Saturation => self.set_s(val),
      ColorUnit::Lightness => self.set_l(val),
    }
  }
}

//
//
// HSLA
//
//

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsla {
  hsl: Hsl,
  alpha: f32,
}

impl std::str::FromStr for Hsla {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Hsla, ParseError> {
    match from_str::hsla(s) {
      Ok(hsla_tuple) => Ok(Hsla::from_tuple(hsla_tuple)),
      Err(err) => Err(err),
    }
  }
}
impl AlphaColor for Hsla {
  fn get_alpha(&self) -> f32 {
    self.alpha
  }
  fn set_alpha(&self, a: f32) -> Hsla {
    let (h, s, l, _) = self.as_tuple();
    Hsla { hsl: Hsl::from_tuple((h, s, l)), alpha: normalize_ratio(a) }
  }
  fn opacify(&self, a: f32) -> Hsla {
    self.set_alpha(self.alpha + a)
  }
}

impl Color for Hsla {
  type Tuple = ColorTupleA;

  fn new() -> Hsla {
    Hsla { hsl: Hsl::new(), alpha: 1.0 }
  }

  fn to_rgb(&self) -> Rgb {
    self.hsl.to_rgb()
  }
  fn to_rgba(&self) -> Rgba {
    self.to_rgb().to_rgba()
  }
  fn to_hsl(&self) -> Hsl {
    self.hsl
  }
  fn to_hsla(&self) -> Hsla {
    *self
  }
  fn to_css_string(&self) -> String {
    let (h, s, l) = as_rounded_hsl_tuple(&self.hsl.as_tuple());
    format!("hsla({},{}%,{}%,{})", h, s, l, round_ratio(self.alpha))
  }

  fn from_tuple(t: ColorTupleA) -> Hsla {
    let (h, s, l, a) = t;
    Hsla { hsl: Hsl::from_tuple((h, s, l)), alpha: normalize_ratio(a) }
  }
  fn as_tuple(&self) -> ColorTupleA {
    let (h, s, l) = self.hsl.as_tuple();
    (h, s, l, self.alpha)
  }
  fn lighten(&self, amt: f32) -> Hsla {
    Hsla { hsl: self.hsl.lighten(amt), alpha: self.alpha }
  }
  fn saturate(&self, amt: f32) -> Hsla {
    Hsla { hsl: self.hsl.saturate(amt), alpha: self.alpha }
  }
  fn adjust_hue(&self, amt: f32) -> Hsla {
    Hsla { hsl: self.hsl.adjust_hue(amt), alpha: self.alpha }
  }
  fn adjust_color(&self, name: RgbColor, val: f32) -> Hsla {
    Hsla { hsl: self.hsl.adjust_color(name, val), alpha: self.alpha }
  }
  fn get_unit(&self, unit: ColorUnit) -> f32 {
    self.hsl.get_unit(unit)
  }
  fn set_unit(&self, unit: ColorUnit, val: f32) -> Hsla {
    Hsla { hsl: self.hsl.set_unit(unit, val), alpha: self.alpha }
  }
}
