#[cfg(test)]
mod tests;

mod from;
mod iter;
mod ops;

mod from_str;
mod grayscale;

use crate::consts::RGB_UNIT_MAX;
use crate::err::ParseError;
use crate::normalize::{normalize_ratio, normalize_rgb_unit};
use crate::{converters, ColorTuple, Hsl, SaturationInSpace};

use grayscale::rgb_grayscale;
pub use grayscale::GrayScaleMethod;

use iter::RgbIter;

#[derive(Debug, PartialEq, Clone)]
pub struct Rgb {
  r: f32,
  g: f32,
  b: f32,
  a: Option<f32>,
}

impl Rgb {
  fn _apply_tuple(&mut self, t: &ColorTuple) {
    self.r = t.0;
    self.g = t.1;
    self.b = t.2;
  }

  pub fn new(r: f32, g: f32, b: f32, a: Option<f32>) -> Rgb {
    let n = normalize_rgb_unit;

    let a = a.map(normalize_ratio);
    Rgb { r: n(r), g: n(g), b: n(b), a }
  }

  pub fn from_hex_str(s: &str) -> Result<Rgb, ParseError> {
    let tuple = from_str::hex(s)?;
    Ok(Rgb::from(&tuple))
  }

  pub fn get_red(&self) -> f32 {
    self.r
  }
  pub fn get_green(&self) -> f32 {
    self.g
  }
  pub fn get_blue(&self) -> f32 {
    self.b
  }
  pub fn get_alpha(&self) -> f32 {
    self.a.unwrap_or(1.0)
  }

  pub fn set_red(&mut self, val: f32) {
    self.r = normalize_rgb_unit(val);
  }
  pub fn set_green(&mut self, val: f32) {
    self.g = normalize_rgb_unit(val);
  }
  pub fn set_blue(&mut self, val: f32) {
    self.b = normalize_rgb_unit(val);
  }
  pub fn set_alpha(&mut self, val: f32) {
    self.a = Some(normalize_ratio(val));
  }

  pub fn to_css_string(&self) -> String {
    converters::rgb_to_css_string(self)
  }

  pub fn lighten(&mut self, amt: f32) {
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

  pub fn adjust_hue(&mut self, hue: f32) {
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
