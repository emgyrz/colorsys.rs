mod converters;

use crate::consts::{ALL_MIN, HUE_MAX};
use crate::normalize::{bound_hue, normalize_hue, normalize_percent, normalize_ratio};

use crate::common::hsl_hsv_from_str;
use crate::{ColorTuple, ColorTupleA, ParseError, Rgb, SaturationInSpace};
use converters::hsl_to_rgb;

#[derive(Debug, PartialEq, Clone)]
pub struct Hsl {
  h: f32,
  s: f32,
  l: f32,
  a: Option<f32>,
}

impl Hsl {
  pub fn default() -> Hsl {
    Hsl { h: 0.0, s: 0.0, l: 0.0, a: None }
  }

  pub fn from(h: f32, s: f32, l: f32) -> Hsl {
    Hsl { h: normalize_hue(h), s: normalize_percent(s), l: normalize_percent(l), a: None }
  }
  pub fn from_with_alpha(h: f32, s: f32, l: f32, a: f32) -> Hsl {
    let a = Some(normalize_ratio(a));
    Hsl { h: normalize_hue(h), s: normalize_percent(s), l: normalize_percent(l), a }
  }

  pub fn from_tuple(t: &ColorTuple) -> Hsl {
    Hsl::from(t.0, t.1, t.2)
  }

  pub fn from_tuple_with_alpha(t: &ColorTupleA) -> Hsl {
    Hsl::from_with_alpha(t.0, t.1, t.2, t.3)
  }

  pub fn as_tuple(&self) -> ColorTuple {
    (self.h, self.s, self.l)
  }

  pub fn as_tuple_with_alpha(&self) -> ColorTupleA {
    (self.h, self.s, self.l, self.get_alpha())
  }

  pub fn get_hue(&self) -> f32 {
    self.h
  }
  pub fn get_saturation(&self) -> f32 {
    self.s
  }
  pub fn get_lightness(&self) -> f32 {
    self.l
  }
  pub fn get_alpha(&self) -> f32 {
    self.a.unwrap_or(1.0)
  }

  pub fn set_hue(&mut self, val: f32) {
    self.h = normalize_hue(val);
  }
  pub fn set_saturation(&mut self, val: f32) {
    self.s = normalize_percent(val);
  }
  pub fn set_lightness(&mut self, val: f32) {
    self.l = normalize_percent(val);
  }
  pub fn set_alpha(&mut self, val: f32) {
    self.a = Some(normalize_ratio(val));
  }

  pub fn to_rgb(&self) -> Rgb {
    Rgb::from_tuple(&hsl_to_rgb(&self.as_tuple()))
  }

  pub fn lighten(&mut self, amt: f32) {
    self.set_lightness(self.l + amt)
  }

  pub fn saturate(&mut self, sat: SaturationInSpace) {
    match sat {
      SaturationInSpace::Hsl(amt) => self.set_saturation(self.s + amt),
      SaturationInSpace::Hsv(amt) => {
        println!("{}", amt);
        unimplemented!();
      }
    }
  }

  pub fn adjust_hue(&mut self, hue: f32) {
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

impl std::str::FromStr for Hsl {
  type Err = ParseError;
  fn from_str(s: &str) -> Result<Hsl, ParseError> {
    let (tuple, alpha) = hsl_hsv_from_str(s, "hsl")?;
    let mut hsl = Hsl::from_tuple(&tuple);
    if let Some(a) = alpha {
      hsl.set_alpha(a);
    }
    Ok(hsl)
  }
}
