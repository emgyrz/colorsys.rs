use super::{Hsla, Rgb, Rgba};
use crate::converters::{as_rounded_hsl_tuple, hsl_to_rgb, invert_hue};
use crate::error::ParseError;
use crate::normalize::{normalize_hsl, normalize_hue, normalize_percent};
use crate::{from_str, Color, ColorTuple, RgbColor};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsl {
  h: f32,
  s: f32,
  l: f32,
}

impl Hsl {
  pub fn from(h: f32, s: f32, l: f32) -> Hsl {
    Hsl::from_tuple(&(h, s, l))
  }

  pub fn grayscale(&self) -> Hsl {
    Hsl { h: 0.0, s: 0.0, l: self.l }
  }

}

impl std::str::FromStr for Hsl {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Hsl, ParseError> {
    match from_str::hsl(s) {
      Ok(hsl_tuple) => Ok(Hsl::from_tuple(&hsl_tuple)),
      Err(err) => Err(err),
    }
  }
}

impl Color for Hsl {
  type Tuple = ColorTuple;

  fn new() -> Hsl {
    Hsl { h: 0.0, s: 0.0, l: 0.0 }
  }

  fn get_hue(&self) -> f32 {
    self.h
  }
  fn get_saturation(&self) -> f32 {
    self.s
  }
  fn get_lightness(&self) -> f32 {
    self.l
  }
  fn set_hue(&self, val: f32) -> Hsl {
    Hsl { h: normalize_hue(val), s: self.s, l: self.l }
  }
  fn set_saturation(&self, val: f32) -> Hsl {
    Hsl { h: self.h, s: normalize_percent(val), l: self.l }
  }
  fn set_lightness(&self, val: f32) -> Hsl {
    Hsl { h: self.h, s: self.s, l: normalize_percent(val) }
  }

  fn get_red(&self) -> f32 {
    self.to_rgb().get_red()
  }
  fn get_green(&self) -> f32 {
    self.to_rgb().get_green()
  }
  fn get_blue(&self) -> f32 {
    self.to_rgb().get_blue()
  }
  fn set_red(&self, val: f32) -> Hsl {
    self.to_rgb().set_red(val).to_hsl()
  }
  fn set_green(&self, val: f32) -> Hsl {
    self.to_rgb().set_green(val).to_hsl()
  }
  fn set_blue(&self, val: f32) -> Hsl {
    self.to_rgb().set_blue(val).to_hsl()
  }

  fn to_rgb(&self) -> Rgb {
    Rgb::from_tuple(&hsl_to_rgb(&self.as_tuple()))
  }
  fn to_rgba(&self) -> Rgba {
    self.to_rgb().to_rgba()
  }
  fn to_hsl(&self) -> Hsl {
    *self
  }
  fn to_hsla(&self) -> Hsla {
    let (h, s, l) = self.as_tuple();
    Hsla::from_tuple(&(h, s, l, 1.0))
  }
  fn to_css_string(&self) -> String {
    let (h, s, l) = as_rounded_hsl_tuple(&self.as_tuple());
    format!("hsl({},{}%,{}%)", h, s, l)
  }
  fn from_tuple(t: &ColorTuple) -> Hsl {
    let (h, s, l) = normalize_hsl(&t);
    Hsl { h, s, l }
  }
  fn as_tuple(&self) -> ColorTuple {
    (self.h, self.s, self.l)
  }
  fn lighten(&self, val: f32) -> Hsl {
    self.set_lightness(self.l + val)
  }
  fn saturate(&self, val: f32) -> Hsl {
    self.set_saturation(self.s + val)
  }
  fn adjust_hue(&self, hue: f32) -> Hsl {
    self.set_hue(self.h + hue)
  }
  fn adjust_color(&self, name: RgbColor, val: f32) -> Hsl {
    self.to_rgb().adjust_color(name, val).to_hsl()
  }

  fn invert(&self) -> Hsl {
    Hsl { h: invert_hue(self.h), s: self.s, l: self.l }
  }
}
