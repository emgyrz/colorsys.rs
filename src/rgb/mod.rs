#[cfg(test)]
mod tests;

use crate::common::approx::approx_def;
use crate::consts::{RATIO_MAX, RGB_UNIT_MAX};
use crate::err::ParseError;
use crate::normalize::{normalize_ratio, normalize_rgb_unit};
use crate::{converters, ColorTuple, Hsl, SaturationInSpace};

mod from;
mod from_str;
mod grayscale;
mod iter;
mod ops;

use grayscale::rgb_grayscale;
pub use grayscale::GrayScaleMethod;
use iter::RgbIter;

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

    let a = a.map(normalize_ratio).filter(|al| approx_def(*al, RATIO_MAX));
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
  pub fn get_alpha(&self) -> f64 {
    self.a.unwrap_or(1.0)
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
  pub fn set_alpha(&mut self, val: f64) {
    self.a = Some(normalize_ratio(val));
  }

  pub fn to_css_string(&self) -> String {
    converters::rgb_to_css_string(self)
  }

  pub fn lighten(&mut self, amt: f64) {
    let mut hsl: Hsl = self.into();
    hsl.lighten(amt);
    let lightened_rgb = hsl.to_rgb();
    self._apply_tuple(&lightened_rgb.into());
  }

  pub fn saturate(&mut self, sat: SaturationInSpace) {
    match sat {
      SaturationInSpace::Hsl(amt) => {
        let mut hsl: Hsl = self.into();
        hsl.set_saturation(hsl.get_saturation() + amt);
        self._apply_tuple(&hsl.to_rgb().into());
      }
      SaturationInSpace::Hsv(amt) => {
        println!("{}", amt);
        unimplemented!();
      }
    }
  }

  pub fn adjust_hue(&mut self, hue: f64) {
    let mut hsl: Hsl = self.into();
    hsl.adjust_hue(hue);
    self._apply_tuple(&hsl.to_rgb().into());
  }

  pub fn grayscale(&mut self, method: GrayScaleMethod) {
    rgb_grayscale(self, method);
  }

  pub fn invert(&mut self) {
    self.r = RGB_UNIT_MAX - self.r;
    self.g = RGB_UNIT_MAX - self.g;
    self.b = RGB_UNIT_MAX - self.b;
  }

  pub fn iter(&self) -> RgbIter {
    RgbIter::new(self.into(), self.a)
  }
}

impl Default for Rgb {
  fn default() -> Rgb {
    Rgb { r: 0.0, g: 0.0, b: 0.0, a: None }
  }
}

impl AsRef<Rgb> for Rgb {
  fn as_ref(&self) -> &Rgb {
    &self
  }
}

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
