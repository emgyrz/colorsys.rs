#[cfg(test)]
mod tests;

mod converters;
mod from;

use crate::common::approx::approx_def;
use crate::consts::{ALL_MIN, HUE_MAX, RATIO_MAX};
use crate::normalize::{bound_hue, normalize_hue, normalize_percent, normalize_ratio};

use crate::common::{hsl_hsv_from_str, Hs};
use crate::{ColorTuple, ColorTupleA, ParseError, Rgb, SaturationInSpace};
use converters::hsl_to_rgb;

#[derive(Debug, PartialEq, Clone)]
pub struct Hsl {
  h: f64,
  s: f64,
  l: f64,
  a: Option<f64>,
}

impl Hsl {
  pub fn new(h: f64, s: f64, l: f64, a: Option<f64>) -> Hsl {
    let a = a.map(normalize_ratio).filter(|al| approx_def(*al, RATIO_MAX));
    Hsl { h: normalize_hue(h), s: normalize_percent(s), l: normalize_percent(l), a }
  }

  pub fn as_tuple(&self) -> ColorTuple {
    (self.h, self.s, self.l)
  }

  pub fn as_tuple_with_alpha(&self) -> ColorTupleA {
    (self.h, self.s, self.l, self.get_alpha())
  }

  pub fn get_hue(&self) -> f64 {
    self.h
  }
  pub fn get_saturation(&self) -> f64 {
    self.s
  }
  pub fn get_lightness(&self) -> f64 {
    self.l
  }
  pub fn get_alpha(&self) -> f64 {
    self.a.unwrap_or(1.0)
  }

  pub fn set_hue(&mut self, val: f64) {
    self.h = normalize_hue(val);
  }
  pub fn set_saturation(&mut self, val: f64) {
    self.s = normalize_percent(val);
  }
  pub fn set_lightness(&mut self, val: f64) {
    self.l = normalize_percent(val);
  }
  pub fn set_alpha(&mut self, val: f64) {
    self.a = Some(normalize_ratio(val));
  }

  pub fn to_rgb(&self) -> Rgb {
    Rgb::from(&hsl_to_rgb(&self.as_tuple()))
  }

  pub fn lighten(&mut self, amt: f64) {
    self.set_lightness(self.l + amt)
  }

  pub fn saturate(&mut self, sat: SaturationInSpace) {
    match sat {
      SaturationInSpace::Hsl(s) => self.set_saturation(self.s + s),
      SaturationInSpace::Hsv(s) => {
        println!("{}", s);
        unimplemented!();
      }
    }
  }

  pub fn adjust_hue(&mut self, hue: f64) {
    self.h = bound_hue(self.h + hue);
  }

  pub fn grayscale(&mut self) {
    self.h = ALL_MIN;
    self.s = ALL_MIN;
  }
  pub fn invert(&mut self) {
    self.h = (self.h + HUE_MAX * 0.5) % HUE_MAX
  }
}

impl Default for Hsl {
  fn default() -> Hsl {
    Hsl { h: 0.0, s: 0.0, l: 0.0, a: None }
  }
}

impl std::str::FromStr for Hsl {
  type Err = ParseError;
  fn from_str(s: &str) -> Result<Hsl, ParseError> {
    let (tuple, alpha) = hsl_hsv_from_str(s, Hs::Hsl)?;
    let mut hsl = Hsl::from(&tuple);
    if let Some(a) = alpha {
      hsl.set_alpha(a);
    }
    Ok(hsl)
  }
}
