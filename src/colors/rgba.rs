use super::{Hsl, Hsla, Rgb};
use crate::converters::{as_rounded_rgb_tuple, round_ratio};
use crate::error::ParseError;
use crate::normalize::normalize_ratio;
use crate::{from_str, AlphaColor, Color, ColorTupleA, RgbColor};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
  rgb: Rgb,
  alpha: f32,
}

impl std::str::FromStr for Rgba {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Rgba, ParseError> {
    match from_str::rgba(s) {
      Ok(rgba_tuple) => Ok(Rgba::from_tuple(&rgba_tuple)),
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
    Rgba { rgb: Rgb::from_tuple(&(r, g, b)), alpha: normalize_ratio(a) }
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

  fn get_red(&self) -> f32 {
    self.rgb.get_red()
  }
  fn get_green(&self) -> f32 {
    self.rgb.get_green()
  }
  fn get_blue(&self) -> f32 {
    self.rgb.get_blue()
  }
  fn set_red(&self, val: f32) -> Rgba {
    Rgba { rgb: self.rgb.set_red(val), alpha: self.alpha }
  }
  fn set_green(&self, val: f32) -> Rgba {
    Rgba { rgb: self.rgb.set_green(val), alpha: self.alpha }
  }
  fn set_blue(&self, val: f32) -> Rgba {
    Rgba { rgb: self.rgb.set_blue(val), alpha: self.alpha }
  }

  fn get_hue(&self) -> f32 {
    self.to_hsl().get_hue()
  }
  fn get_saturation(&self) -> f32 {
    self.to_hsl().get_saturation()
  }
  fn get_lightness(&self) -> f32 {
    self.to_hsl().get_lightness()
  }
  fn set_hue(&self, val: f32) -> Rgba {
    self.to_hsla().set_hue(val).to_rgba()
  }
  fn set_saturation(&self, val: f32) -> Rgba {
    self.to_hsla().set_saturation(val).to_rgba()
  }
  fn set_lightness(&self, val: f32) -> Rgba {
    self.to_hsla().set_lightness(val).to_rgba()
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
    self.rgb.to_hsla().set_alpha(self.alpha)
  }
  fn to_css_string(&self) -> String {
    let (r, g, b) = as_rounded_rgb_tuple(&self.rgb.as_tuple());
    format!("rgba({},{},{},{})", r, g, b, round_ratio(self.alpha))
  }

  fn from_tuple(t: &ColorTupleA) -> Rgba {
    let (r, g, b, a) = *t;
    Rgba { rgb: Rgb::from_tuple(&(r, g, b)), alpha: normalize_ratio(a) }
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
  fn grayscale(&self) -> Rgba {
    Rgba { rgb: self.rgb.grayscale(), alpha: self.alpha }
  }
}
