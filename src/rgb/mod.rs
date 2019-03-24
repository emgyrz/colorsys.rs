#[cfg(test)]
mod tests;

mod converters;
mod from_str;
mod grayscale;

use crate::consts::RGB_UNIT_MAX;
use crate::err::ParseError;
use crate::normalize::{normalize_ratio, normalize_rgb_unit};
use crate::{ColorTuple, ColorTupleA, Hsl, SaturationInSpace};

use grayscale::rgb_grayscale;
pub use grayscale::GrayScaleMethod;

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

  pub fn default() -> Rgb {
    Rgb { r: 0.0, g: 0.0, b: 0.0, a: None }
  }

  pub fn from(r: f32, g: f32, b: f32) -> Rgb {
    let n = normalize_rgb_unit;
    Rgb { r: n(r), g: n(g), b: n(b), a: None }
  }
  pub fn from_with_alpha(r: f32, g: f32, b: f32, a: f32) -> Rgb {
    let n = normalize_rgb_unit;
    Rgb { r: n(r), g: n(g), b: n(b), a: Some(normalize_ratio(a)) }
  }

  pub fn from_tuple(t: &ColorTuple) -> Rgb {
    Rgb::from(t.0, t.1, t.2)
  }

  pub fn from_tuple_with_alpha(t: &ColorTupleA) -> Rgb {
    Rgb::from_with_alpha(t.0, t.1, t.2, t.3)
  }

  pub fn from_hex_str(s: &str) -> Result<Rgb, ParseError> {
    let tuple = from_str::hex(s)?;
    Ok(Rgb::from_tuple(&tuple))
  }

  pub fn as_tuple(&self) -> ColorTuple {
    (self.r, self.g, self.b)
  }

  pub fn as_tuple_with_alpha(&self) -> ColorTupleA {
    (self.r, self.g, self.b, self.get_alpha())
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

  pub fn to_hsl(&self) -> Hsl {
    let hsl_tuple = converters::rgb_to_hsl(&self.as_tuple());
    Hsl::from_tuple(&hsl_tuple)
  }

  pub fn lighten(&mut self, amt: f32) {
    let mut hsl = self.to_hsl();
    hsl.lighten(amt);
    let lightened_rgb = hsl.to_rgb();
    self._apply_tuple(&lightened_rgb.as_tuple());
  }

  pub fn saturate(&mut self, sat: SaturationInSpace) {
    match sat {
      SaturationInSpace::Hsl(amt) => {
        let mut hsl = self.to_hsl();
        hsl.set_saturation(hsl.get_saturation() + amt);
        self._apply_tuple(&hsl.to_rgb().as_tuple());
      }
      SaturationInSpace::Hsv(amt) => {
        println!("{}", amt);
        unimplemented!();
      }
    }
  }

  pub fn adjust_hue(&mut self, hue: f32) {
    let mut hsl = self.to_hsl();
    hsl.adjust_hue(hue);
    self._apply_tuple(&hsl.to_rgb().as_tuple());
  }

  pub fn grayscale(&mut self, method: GrayScaleMethod) {
    rgb_grayscale(self, method);
  }

  pub fn invert(&mut self) {
    self.r = RGB_UNIT_MAX - self.r;
    self.g = RGB_UNIT_MAX - self.g;
    self.b = RGB_UNIT_MAX - self.b;
  }
}

impl std::str::FromStr for Rgb {
  type Err = ParseError;
  fn from_str(s: &str) -> Result<Rgb, ParseError> {
    let (tuple, alpha) = from_str::rgb(s)?;
    let mut rgb = Rgb::from_tuple(&tuple);
    if let Some(a) = alpha {
      rgb.set_alpha(a);
    }
    Ok(rgb)
  }
}
