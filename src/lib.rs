mod converters;
mod error;
mod from_str;
mod misc;
mod normalize;

#[cfg(test)]
mod tests;

use converters::{
  as_rounded_hsl_tuple, as_rounded_rgb_tuple, hsl_to_rgb, rgb_to_hex, rgb_to_hsl, round_ratio,
};
use normalize::{normalize_hsl, normalize_hue, normalize_ratio, normalize_rgb, normalize_rgb_unit};

use misc::get_unit;

pub use error::ParseError;

pub type ColorTuple = (f32, f32, f32);
pub type ColorTupleA = (f32, f32, f32, f32);

pub trait Color {
  type Tuple;
  fn to_hex(&self) -> Hex;
  fn to_rgb(&self) -> Rgb;
  fn to_rgba(&self) -> Rgba;
  fn to_hsl(&self) -> Hsl;
  fn to_hsla(&self) -> Hsla;
  fn to_css(&self) -> String;
  fn from_tuple(tuple: Self::Tuple) -> Self;
  fn as_tuple(&self) -> Self::Tuple;
  fn lighten(&self, amt: f32) -> Self;
  fn saturate(&self, amt: f32) -> Self;
  fn adjust_hue(&self, amt: f32) -> Self;
  fn adjust_color(&self, col_name: RgbColor, val: f32) -> Self;
  fn get_unit(&self, unit: ColorUnit) -> f32;
}

pub trait AlphaColor {
  fn opacify(&self, o: f32) -> Self;
  fn get_alpha(&self) -> f32;
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

  fn to_rgb(&self) -> Rgb {
    *self
  }
  fn to_rgba(&self) -> Rgba {
    let (r, g, b) = self.as_tuple();
    Rgba { rgb: Rgb::from_tuple((r, g, b)), alpha: 1.0 }
  }
  fn to_hex(&self) -> Hex {
    Hex { rgb: *self }
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
  /// use colors_rs::{Rgb,Color};
  ///
  /// assert_eq!(Rgb::from_tuple((225.0,101.7, 21.0)).to_css(), "rgb(225,102,21)");
  /// ```
  fn to_css(&self) -> String {
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
    let (mut r, mut g, mut b) = self.as_tuple();
    match name {
      RgbColor::Red => r = normalize_rgb_unit(r + val),
      RgbColor::Green => g = normalize_rgb_unit(g + val),
      RgbColor::Blue => b = normalize_rgb_unit(b + val),
    }
    Rgb::from_tuple((r, g, b))
  }
  fn get_unit(&self, unit: ColorUnit) -> f32 {
    get_unit(*self, unit)
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
  fn opacify(&self, alpha: f32) -> Rgba {
    let (r, g, b, a) = self.as_tuple();
    Rgba { rgb: Rgb::from_tuple((r, g, b)), alpha: normalize_ratio(a + alpha) }
  }
  fn get_alpha(&self) -> f32 {
    self.alpha
  }
}

impl Color for Rgba {
  type Tuple = ColorTupleA;

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
  fn to_hex(&self) -> Hex {
    Hex { rgb: self.to_rgb() }
  }

  fn to_css(&self) -> String {
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
    get_unit(*self, unit)
  }
}

//
//
// HEX
//
//
pub type HexTuple = (String, String, String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hex {
  rgb: Rgb,
}

impl std::str::FromStr for Hex {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Hex, ParseError> {
    match from_str::hex(s) {
      Ok(rgb_tuple) => Ok(Hex { rgb: Rgb::from_tuple(rgb_tuple) }),
      Err(err) => Err(err),
    }
  }
}

impl Color for Hex {
  type Tuple = HexTuple;

  fn to_rgb(&self) -> Rgb {
    self.rgb
  }
  fn to_rgba(&self) -> Rgba {
    self.rgb.to_rgba()
  }
  fn to_hex(&self) -> Hex {
    *self
  }
  fn to_hsl(&self) -> Hsl {
    self.to_rgb().to_hsl()
  }
  fn to_hsla(&self) -> Hsla {
    self.to_rgb().to_hsla()
  }
  fn to_css(&self) -> String {
    let (r, g, b) = self.as_tuple();
    format!("#{}{}{}", r, g, b)
  }

  fn from_tuple(t: HexTuple) -> Hex {
    // TODO:
    Hex { rgb: Rgb::from_tuple((0.0, 0.0, 0.0)) }
  }
  fn as_tuple(&self) -> HexTuple {
    rgb_to_hex(&self.rgb.as_tuple())
  }

  fn lighten(&self, amt: f32) -> Hex {
    Hex { rgb: self.rgb.lighten(amt) }
  }
  fn saturate(&self, amt: f32) -> Hex {
    Hex { rgb: self.rgb.saturate(amt) }
  }
  fn adjust_hue(&self, amt: f32) -> Hex {
    Hex { rgb: self.rgb.adjust_hue(amt) }
  }
  fn adjust_color(&self, name: RgbColor, val: f32) -> Hex {
    Hex { rgb: self.rgb.adjust_color(name, val) }
  }
  fn get_unit(&self, unit: ColorUnit) -> f32 {
    get_unit(*self, unit)
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
  fn to_hex(&self) -> Hex {
    self.to_rgb().to_hex()
  }

  fn to_css(&self) -> String {
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
  fn lighten(&self, amt: f32) -> Hsl {
    Hsl { h: self.h, s: self.s, l: normalize_ratio(self.l + amt) }
  }
  fn saturate(&self, amt: f32) -> Hsl {
    Hsl { h: self.h, s: normalize_ratio(self.s + amt), l: self.l }
  }
  fn adjust_hue(&self, hue: f32) -> Hsl {
    Hsl { h: normalize_hue(self.h + hue), s: self.s, l: self.l }
  }
  fn adjust_color(&self, name: RgbColor, val: f32) -> Hsl {
    self.to_rgb().adjust_color(name, val).to_hsl()
  }
  fn get_unit(&self, unit: ColorUnit) -> f32 {
    get_unit(*self, unit)
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
  fn opacify(&self, alpha: f32) -> Hsla {
    let (h, s, l, a) = self.as_tuple();
    Hsla { hsl: Hsl::from_tuple((h, s, l)), alpha: normalize_ratio(a + alpha) }
  }
  fn get_alpha(&self) -> f32 {
    self.alpha
  }
}

impl Color for Hsla {
  type Tuple = ColorTupleA;

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
  fn to_hex(&self) -> Hex {
    self.to_rgb().to_hex()
  }

  fn to_css(&self) -> String {
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
    get_unit(*self, unit)
  }
}
