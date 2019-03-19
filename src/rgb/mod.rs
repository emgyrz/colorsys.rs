mod grayscale;

use crate::consts::RGB_UNIT_MAX;
use crate::normalize::{normalize_alpha, normalize_rgb_unit};
use crate::{ColorTuple, ColorTupleA};

use grayscale::rgb_grayscale;
pub use grayscale::GrayScaleMethod;

#[derive(Debug, PartialEq)]
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
    Rgb { r: n(r), g: n(g), b: n(b), a: Some(normalize_alpha(a)) }
  }

  pub fn from_tuple(t: &ColorTuple) -> Rgb {
    Rgb::from(t.0, t.1, t.2)
  }

  pub fn from_tuple_with_alpha(t: &ColorTupleA) -> Rgb {
    Rgb::from_with_alpha(t.0, t.1, t.2, t.3)
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
    self.a = Some(normalize_alpha(val));
  }

  pub fn grayscale(&mut self, method: GrayScaleMethod) {
    let grayscaled = rgb_grayscale(&self.as_tuple(), method);
    self._apply_tuple(&grayscaled);
  }

  pub fn invert(&mut self) {
    self.r = RGB_UNIT_MAX - self.r;
    self.g = RGB_UNIT_MAX - self.g;
    self.b = RGB_UNIT_MAX - self.b;
  }
}
