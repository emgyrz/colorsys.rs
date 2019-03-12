use super::{Hsl, Rgb, Rgba};
use crate::converters::{as_rounded_hsl_tuple, round_ratio};
use crate::error::ParseError;
use crate::normalize::normalize_ratio;
use crate::{from_str, AlphaColor, Color, ColorTupleA, RgbColor};

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

  fn get_hue(&self) -> f32 {
    self.hsl.get_hue()
  }
  fn get_saturation(&self) -> f32 {
    self.hsl.get_saturation()
  }
  fn get_lightness(&self) -> f32 {
    self.hsl.get_lightness()
  }
  fn set_hue(&self, val: f32) -> Hsla {
    Hsla { hsl: self.hsl.set_hue(val), alpha: self.alpha }
  }
  fn set_saturation(&self, val: f32) -> Hsla {
    Hsla { hsl: self.hsl.set_saturation(val), alpha: self.alpha }
  }
  fn set_lightness(&self, val: f32) -> Hsla {
    Hsla { hsl: self.hsl.set_lightness(val), alpha: self.alpha }
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
  fn set_red(&self, val: f32) -> Hsla {
    self.to_rgba().set_red(val).to_hsla()
  }
  fn set_green(&self, val: f32) -> Hsla {
    self.to_rgba().set_green(val).to_hsla()
  }
  fn set_blue(&self, val: f32) -> Hsla {
    self.to_rgba().set_blue(val).to_hsla()
  }

  fn to_rgb(&self) -> Rgb {
    self.hsl.to_rgb()
  }
  fn to_rgba(&self) -> Rgba {
    self.to_rgb().to_rgba().set_alpha(self.alpha)
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
  fn grayscale(&self) -> Hsla {
    Hsla { hsl: self.hsl.grayscale(), alpha: self.alpha }
  }
}
