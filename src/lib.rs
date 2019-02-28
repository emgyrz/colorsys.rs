mod converters;
mod normalize;

#[cfg(test)]
mod tests;

use converters::{
  as_rounded_hsl_tuple, as_rounded_rgb_tuple, hsl_to_rgb, rgb_to_hex, rgb_to_hsl, round_ratio,
};
use normalize::{normalize_hsl, normalize_hue, normalize_ratio, normalize_rgb, normalize_rgb_unit};

pub trait Color {
  type Tuple;
  fn to_hex(&self) -> Hex;
  fn to_rgb(&self) -> Rgb;
  fn to_rgba(&self) -> Rgba;
  fn to_hsl(&self) -> Hsl;
  fn to_hsla(&self) -> Hsla;
  fn to_css(&self) -> String;
  fn from_tuple(tuple: Self::Tuple) -> Self;
  fn as_tuple(&self) -> Self::Tuple;
  fn lighten(&self, amt: f32) -> Self;
  fn saturate(&self, amt: f32) -> Self;
  fn adjust_hue(&self, amt: f32) -> Self;
  fn adjust_color(&self, col_name: RgbColor, val: f32) -> Self;
}

#[derive(Clone, Copy)]
pub enum RgbColor {
  Red,
  Green,
  Blue,
}

//
//
// RGB
//
//
pub type RgbTuple = (f32, f32, f32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb {
  r: f32,
  g: f32,
  b: f32,
}

impl Color for Rgb {
  type Tuple = RgbTuple;

  fn to_rgb(&self) -> Rgb {
    *self
  }
  fn to_rgba(&self) -> Rgba {
    let (r, g, b) = self.as_tuple();
    Rgba { rgb: Rgb::from_tuple((r, g, b)), alpha: 1.0 }
  }
  fn to_hex(&self) -> Hex {
    Hex { rgb: *self }
  }
  fn to_hsl(&self) -> Hsl {
    Hsl::from_tuple(rgb_to_hsl(&self.as_tuple()))
  }
  fn to_hsla(&self) -> Hsla {
    self.to_hsl().to_hsla()
  }
  fn to_css(&self) -> String {
    let (r, g, b) = as_rounded_rgb_tuple(&self.as_tuple());
    format!("rgb({},{},{})", r, g, b)
  }

  fn from_tuple(t: RgbTuple) -> Rgb {
    let (r, g, b) = normalize_rgb(&t);
    Rgb { r, g, b }
  }
  fn as_tuple(&self) -> RgbTuple {
    (self.r, self.g, self.b)
  }

  fn lighten(&self, amt: f32) -> Rgb {
    self.to_hsl().lighten(amt).to_rgb()
  }
  fn saturate(&self, amt: f32) -> Rgb {
    self.to_hsl().saturate(amt).to_rgb()
  }
  fn adjust_hue(&self, amt: f32) -> Rgb {
    self.to_hsl().adjust_hue(amt).to_rgb()
  }
  fn adjust_color(&self, name: RgbColor, val: f32) -> Rgb {
    let (mut r, mut g, mut b) = self.as_tuple();
    match name {
      RgbColor::Red => r = normalize_rgb_unit(r + val),
      RgbColor::Green => g = normalize_rgb_unit(g + val),
      RgbColor::Blue => b = normalize_rgb_unit(b + val),
    }
    Rgb::from_tuple((r, g, b))
  }
}

//
//
// RGBA
//
//
pub type RgbaTuple = (f32, f32, f32, f32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
  rgb: Rgb,
  alpha: f32,
}

impl Color for Rgba {
  type Tuple = RgbaTuple;

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
    Hsla { hsl: self.rgb.to_hsl(), alpha: self.alpha }
  }
  fn to_hex(&self) -> Hex {
    Hex { rgb: self.to_rgb() }
  }

  fn to_css(&self) -> String {
    let (r, g, b) = as_rounded_rgb_tuple(&self.rgb.as_tuple());
    format!("rgba({},{},{},{})", r, g, b, round_ratio(self.alpha))
  }

  fn from_tuple(t: RgbaTuple) -> Rgba {
    let (r, g, b, a) = t;
    Rgba { rgb: Rgb::from_tuple((r, g, b)), alpha: normalize_ratio(a) }
  }
  fn as_tuple(&self) -> RgbaTuple {
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
}

//
//
// HEX
//
//
pub type HexTuple = (String, String, String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hex {
  rgb: Rgb,
}

impl Color for Hex {
  type Tuple = HexTuple;

  fn to_rgb(&self) -> Rgb {
    self.rgb
  }
  fn to_rgba(&self) -> Rgba {
    self.rgb.to_rgba()
  }
  fn to_hex(&self) -> Hex {
    *self
  }
  fn to_hsl(&self) -> Hsl {
    self.to_rgb().to_hsl()
  }
  fn to_hsla(&self) -> Hsla {
    self.to_rgb().to_hsla()
  }
  fn to_css(&self) -> String {
    let (r, g, b) = self.as_tuple();
    format!("#{}{}{}", r, g, b)
  }

  fn from_tuple(t: HexTuple) -> Hex {
    // TODO:
    Hex { rgb: Rgb::from_tuple((0.0, 0.0, 0.0)) }
  }
  fn as_tuple(&self) -> HexTuple {
    rgb_to_hex(&self.rgb.as_tuple())
  }

  fn lighten(&self, amt: f32) -> Hex {
    Hex { rgb: self.rgb.lighten(amt) }
  }
  fn saturate(&self, amt: f32) -> Hex {
    Hex { rgb: self.rgb.saturate(amt) }
  }
  fn adjust_hue(&self, amt: f32) -> Hex {
    Hex { rgb: self.rgb.adjust_hue(amt) }
  }
  fn adjust_color(&self, name: RgbColor, val: f32) -> Hex {
    Hex { rgb: self.rgb.adjust_color(name, val) }
  }
}

//
//
// HSL
//
//
pub type HslTuple = (f32, f32, f32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsl {
  h: f32,
  s: f32,
  l: f32,
}

impl Color for Hsl {
  type Tuple = HslTuple;

  fn to_rgb(&self) -> Rgb {
    Rgb::from_tuple(hsl_to_rgb(&self.as_tuple()))
  }
  fn to_rgba(&self) -> Rgba {
    self.to_rgb().to_rgba()
  }
  fn to_hsl(&self) -> Hsl {
    *self
  }
  fn to_hsla(&self) -> Hsla {
    Hsla { hsl: *self, alpha: 1.0 }
  }
  fn to_hex(&self) -> Hex {
    self.to_rgb().to_hex()
  }

  fn to_css(&self) -> String {
    let (h, s, l) = as_rounded_hsl_tuple(&self.as_tuple());
    format!("hsl({},{}%,{}%)", h, s, l)
  }

  fn from_tuple(t: HslTuple) -> Hsl {
    let (h, s, l) = normalize_hsl(&t);
    Hsl { h, s, l }
  }
  fn as_tuple(&self) -> HslTuple {
    (self.h, self.s, self.l)
  }
  fn lighten(&self, amt: f32) -> Hsl {
    Hsl { h: self.h, s: self.s, l: normalize_ratio(self.l + amt) }
  }
  fn saturate(&self, amt: f32) -> Hsl {
    Hsl { h: self.h, s: normalize_ratio(self.s + amt), l: self.l }
  }
  fn adjust_hue(&self, hue: f32) -> Hsl {
    Hsl { h: normalize_hue(self.h + hue), s: self.s, l: self.l }
  }
  fn adjust_color(&self, name: RgbColor, val: f32) -> Hsl {
    self.to_rgb().adjust_color(name, val).to_hsl()
  }
}

//
//
// HSLA
//
//
pub type HslaTuple = (f32, f32, f32, f32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsla {
  hsl: Hsl,
  alpha: f32,
}

impl Color for Hsla {
  type Tuple = HslaTuple;

  fn to_rgb(&self) -> Rgb {
    self.hsl.to_rgb()
  }
  fn to_rgba(&self) -> Rgba {
    self.to_rgb().to_rgba()
  }
  fn to_hsl(&self) -> Hsl {
    self.hsl
  }
  fn to_hsla(&self) -> Hsla {
    *self
  }
  fn to_hex(&self) -> Hex {
    self.to_rgb().to_hex()
  }

  fn to_css(&self) -> String {
    let (h, s, l) = as_rounded_hsl_tuple(&self.hsl.as_tuple());
    format!("hsla({},{}%,{}%,{})", h, s, l, round_ratio(self.alpha))
  }

  fn from_tuple(t: HslaTuple) -> Hsla {
    let (h, s, l, a) = t;
    Hsla { hsl: Hsl::from_tuple((h, s, l)), alpha: normalize_ratio(a) }
  }
  fn as_tuple(&self) -> HslaTuple {
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
}
