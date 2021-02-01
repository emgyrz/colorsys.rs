#[cfg(not(feature = "std"))] use alloc::string::String;

#[cfg(test)]
mod tests;

use crate::common::{approx::approx_def, tuple_to_string, ColorIter};
use crate::consts::RATIO_MAX;
use crate::err::ParseError;
use crate::normalize::{normalize_ratio, normalize_rgb_unit};
use crate::{converters, ColorAlpha, ColorTuple, ColorTupleA, Hsl};

mod from;
mod from_str;
mod grayscale;
mod ops;
mod ratio;
mod transform;

use crate::ratio_converters::rgba_to_ratio;
use grayscale::rgb_grayscale;
pub use grayscale::GrayScaleMethod;
pub use ratio::RgbRatio;

/// The RGB color model.
///
/// Has r (red), g (green), b(blue) and optional a(alpha channel) fields.
/// Red, green, blue values are stored between 0.0 and 255.0, alpha is between 0.0 and 1.0.
/// If inputed or recieved values are exceeds the allowed value, or is less than zero
/// it will be equalize to limit.
///
/// Can be converted into (and from) other color models.
///
///
/// # Example
/// ```
/// use colorsys::{Rgb, Hsl, prelude::*};
/// let mut rgb1 = Rgb::from((100.0, 255.0, 17.0));
/// // Rgb { r: 100.0, g: 255.0, b: 17.0, a: None }
///
/// let green = rgb1.green();
/// // 255.0
///
/// rgb1.set_red(108.3);
/// // Rgb { r: 108.3, g: 255.0, b: 17.0, .. }
///
/// let mut hsl: Hsl = rgb1.into();
/// // ~Hsl { h: 96.98, s: 100.0, l: 53.333, .. }
///
/// hsl.saturate( SaturationInSpace::Hsl(-57.901) );
/// // ~Hsl { h: 96.98, s: 42.099, l: 53.333, .. }
///
/// let mut rgb2 = Rgb::from(&hsl);
/// // ~Rgb { r: 124.34, g: 186.1, b: 85.9, .. }
///
/// let rgb2tuple: (f64,f64,f64) = rgb2.as_ref().into();
/// // (124.34, 186.1,85.9)
///
/// rgb2 += Rgb::from_hex_str("#35f15b").unwrap();;
/// // ~Rgb { r: 177.33, g: 255.0, b: 176.902, .. }
///
/// rgb2.set_green(-150.0);
/// assert_eq!(rgb2.green(), 0.0);
///
/// rgb2.lighten(-13.123);
/// // ~Rgb { r: 110.41, g: 0.0, b: 110.1, .. }
///
/// rgb2.grayscale_simple();
/// // ~Rgb { r: 55.2, g: 55.2, b: 55.2, .. }
///
/// let css_string = rgb2.to_css_string();
/// assert_eq!(css_string, "rgb(55,55,55)");
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub struct Rgb {
  r: f64,
  g: f64,
  b: f64,
  a: Option<f64>,
}

impl Rgb {
  fn _apply_tuple(&mut self, t: &ColorTuple) {
    self.r = t.0;
    self.g = t.1;
    self.b = t.2;
  }

  pub fn new(r: f64, g: f64, b: f64, a: Option<f64>) -> Rgb {
    let n = normalize_rgb_unit;

    let a = a.map(normalize_ratio).filter(|al| !approx_def(*al, RATIO_MAX));
    Rgb { r: n(r), g: n(g), b: n(b), a }
  }

  pub fn from_hex_str(s: &str) -> Result<Rgb, ParseError> {
    let tuple = converters::hex_to_rgb(s)?;
    Ok(Rgb::from(&tuple))
  }

  pub fn to_hex_string(&self) -> String {
    converters::rgb_to_hex(&self.into())
  }

  pub fn red(&self) -> f64 {
    self.r
  }
  pub fn green(&self) -> f64 {
    self.g
  }
  pub fn blue(&self) -> f64 {
    self.b
  }

  #[deprecated(since = "0.7.0", note = "Please use `red` instead")]
  pub fn get_red(&self) -> f64 { self.red() }
  #[deprecated(since = "0.7.0", note = "Please use `green` instead")]
  pub fn get_green(&self) -> f64 {
    self.green()
  }
  #[deprecated(since = "0.7.0", note = "Please use `blue` instead")]
  pub fn get_blue(&self) -> f64 {
    self.blue()
  }

  pub fn set_red(&mut self, val: f64) {
    self.r = normalize_rgb_unit(val);
  }
  pub fn set_green(&mut self, val: f64) {
    self.g = normalize_rgb_unit(val);
  }
  pub fn set_blue(&mut self, val: f64) {
    self.b = normalize_rgb_unit(val);
  }

  pub fn to_css_string(&self) -> String {
    let t: ColorTupleA = self.into();
    tuple_to_string(&t, "rgb")
  }

  pub fn grayscale(&mut self, method: GrayScaleMethod) {
    rgb_grayscale(self, method);
  }

  pub fn iter(&self) -> ColorIter {
    ColorIter::from_tuple_w_alpha(self.into(), self.a)
  }

  pub fn as_ratio(&self) -> RgbRatio {
    let t = rgba_to_ratio(&self.into());
    RgbRatio { r: t.0, g: t.1, b: t.2, a: t.3 }
  }
}

//
//
//
// Default
//
impl Default for Rgb {
  fn default() -> Rgb {
    Rgb { r: 0.0, g: 0.0, b: 0.0, a: None }
  }
}

//
//
//
// AsRef<Rgb>
//
impl AsRef<Rgb> for Rgb {
  fn as_ref(&self) -> &Rgb {
    &self
  }
}

//
//
//
// FromStr
//
impl core::str::FromStr for Rgb {
  type Err = ParseError;
  fn from_str(s: &str) -> Result<Rgb, ParseError> {
    let (tuple, alpha) = from_str::rgb(s)?;
    let mut rgb = Rgb::from(&tuple);
    if let Some(a) = alpha {
      rgb.set_alpha(a);
    }
    Ok(rgb)
  }
}

//
//
//
// ColorAlpha
//
impl ColorAlpha for Rgb {
  fn get_alpha(&self) -> f64 {
    self.a.unwrap_or(1.0)
  }

  fn set_alpha(&mut self, val: f64) {
    self.a = Some(normalize_ratio(val));
  }

  fn opacify(&mut self, val: f64) {
    self.set_alpha(self.get_alpha() + val);
  }
}

//
//
//
// Iter
//
impl<'a> core::iter::IntoIterator for &'a Rgb {
  type Item = f64;
  type IntoIter = ColorIter;
  fn into_iter(self) -> ColorIter {
    self.iter()
  }
}

impl core::iter::IntoIterator for Rgb {
  type Item = f64;
  type IntoIter = ColorIter;
  fn into_iter(self) -> ColorIter {
    self.iter()
  }
}
