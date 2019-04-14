#[cfg(test)]
mod tests;

use crate::common::{approx::approx_def, ColorIter};
use crate::consts::RATIO_MAX;
use crate::err::ParseError;
use crate::normalize::{normalize_ratio, normalize_rgb_unit};
use crate::{converters, ColorAlpha, ColorTuple, Hsl};

mod from;
mod from_str;
mod grayscale;
mod ops;
mod transform;

use grayscale::rgb_grayscale;
pub use grayscale::GrayScaleMethod;

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
    let tuple = from_str::hex(s)?;
    Ok(Rgb::from(&tuple))
  }

  pub fn get_red(&self) -> f64 {
    self.r
  }
  pub fn get_green(&self) -> f64 {
    self.g
  }
  pub fn get_blue(&self) -> f64 {
    self.b
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
    converters::rgb_to_css_string(self)
  }

  pub fn grayscale(&mut self, method: GrayScaleMethod) {
    rgb_grayscale(self, method);
  }

  pub fn iter(&self) -> ColorIter {
    ColorIter::from_tuple_w_alpha(self.into(), self.a)
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
impl std::str::FromStr for Rgb {
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
impl<'a> std::iter::IntoIterator for &'a Rgb {
  type Item = f64;
  type IntoIter = ColorIter;
  fn into_iter(self) -> ColorIter {
    self.iter()
  }
}

impl std::iter::IntoIterator for Rgb {
  type Item = f64;
  type IntoIter = ColorIter;
  fn into_iter(self) -> ColorIter {
    self.iter()
  }
}
