use super::converters::{hex_to_rgb, normalize_hue, normalize_until_one, rgb_to_hex, rgb_to_hsl};
use super::error::{make_parse_err, ParseError};
use super::from_str;

pub trait Color {
  fn to_hex(&self) -> Hex;
  /// Конвертит цвет в RGB
  ///
  /// # Examples
  ///
  /// ```
  /// let hex: Hex = "#7c4f39".parse().unwrap();
  /// let rgb: Rgb = hex.to_rgb();
  /// ```
  fn to_rgb(&self) -> Rgb;
  fn to_rgba(&self) -> Rgba;
  fn to_hsl(&self) -> Hsl;
  fn to_css(&self) -> String;
}

pub trait SetAlpha<T, R> {
  fn set_alpha(&self, a: f32) -> R;
}

pub trait GetAlpha {
  fn get_alpha(&self) -> f32;
}

pub trait SetRgb<T> {
  fn red(&self, val: u8) -> T;
  fn green(&self, val: u8) -> T;
  fn blue(&self, val: u8) -> T;
}

pub trait SetHsl<T> {
  fn hue(&self, val: u16) -> T;
  fn saturation(&self, val: f32) -> T;
  fn lightness(&self, val: f32) -> T;
}

//
//
//  HEX
//
//

pub struct Hex {
  value: String,
  num: usize,
}

impl Hex {
  // pub fn from<T: Color>(col: &T) -> Hex {
  //   col.to_hex()
  // }

  pub fn get_num(&self) -> usize {
    self.num
  }

  pub fn get_val(&self) -> &str {
    &self.value
  }

  // pub fn alpha(&self, a: f32) -> Rgba {
  //   self.to_rgba().alpha(a)
  // }
}

impl Clone for Hex {
  fn clone(&self) -> Hex {
    Hex {
      value: self.value.clone(),
      num: self.num,
    }
  }
}

impl std::str::FromStr for Hex {
  type Err = ParseError;
  /// Пробует распарсить строку как hex.
  ///
  /// # Examples
  ///
  /// ```
  /// use std::str::FromStr;
  /// let color = Hex::from_str("ffcc00").unwrap();
  /// ```
  /// ```
  /// let color: Hex = "de1a7e".parse().unwrap();
  /// ```
  fn from_str(s: &str) -> Result<Hex, ParseError> {
    match from_str::hex(s) {
      Ok((value, num)) => Ok(Hex { value, num }),
      Err(err) => Err(err),
    }
  }
}

impl Color for Hex {
  fn to_hex(&self) -> Hex {
    self.clone()
  }
  fn to_rgb(&self) -> Rgb {
    Rgb::from_tuple(hex_to_rgb(&self))
  }

  fn to_rgba(&self) -> Rgba {
    self.to_rgb().to_rgba()
  }

  fn to_hsl(&self) -> Hsl {
    Hsl {
      h: 0,
      s: 0.0,
      l: 0.0,
    }
  }

  fn to_css(&self) -> String {
    format!("#{}", self.value)
  }
}

//
//
//  Rgb
//
//
pub type RgbTuple = (u8, u8, u8);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl Rgb {
  pub fn as_tuple(self) -> RgbTuple {
    (self.r, self.g, self.b)
  }

  pub fn from_tuple(t: RgbTuple) -> Rgb {
    Rgb {
      r: t.0,
      g: t.1,
      b: t.2,
    }
  }

  fn set_rbg(&self, key: &str, val: u8) -> Rgb {
    let mut tuple = self.as_tuple();
    match key {
      "red" => {
        tuple.0 = val;
      }
      "green" => {
        tuple.1 = val;
      }
      "blue" => {
        tuple.2 = val;
      }
      _ => {}
    }
    Rgb::from_tuple(tuple)
  }
}

impl Color for Rgb {
  fn to_hex(&self) -> Hex {
    let (value, num) = rgb_to_hex(&self);
    Hex { value, num }
  }
  fn to_rgb(&self) -> Rgb {
    *self
  }

  fn to_rgba(&self) -> Rgba {
    Rgba {
      r: self.r,
      g: self.g,
      b: self.b,
      a: 1.0,
    }
  }

  fn to_hsl(&self) -> Hsl {
    rgb_to_hsl(&self)
  }

  fn to_css(&self) -> String {
    format!("rgb({},{},{})", self.r, self.g, self.b)
  }
}

impl SetAlpha<Rgb, Rgba> for Rgb {
  fn set_alpha(&self, a: f32) -> Rgba {
    self.to_rgba().set_alpha(a)
  }
}

impl SetRgb<Rgb> for Rgb {
  fn red(&self, val: u8) -> Rgb {
    self.set_rbg("red", val)
  }
  fn green(&self, val: u8) -> Rgb {
    self.set_rbg("green", val)
  }
  fn blue(&self, val: u8) -> Rgb {
    self.set_rbg("blue", val)
  }
}

//
//
//  Rgba
//
//
type RgbaTuple = (u8, u8, u8, f32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  a: f32,
}

impl Rgba {
  pub fn as_tuple(&self) -> RgbaTuple {
    (self.r, self.g, self.b, self.a)
  }

  pub fn from_tuple(t: RgbaTuple) -> Rgba {
    Rgba {
      r: t.0,
      g: t.1,
      b: t.2,
      a: normalize_until_one(t.3),
    }
  }

  fn set_rbg(&self, key: &str, val: u8) -> Rgba {
    let a = self.a;
    let rgb = self.to_rgb().set_rbg(key, val);
    Rgba {
      r: rgb.r,
      g: rgb.g,
      b: rgb.b,
      a,
    }
  }
}

impl Color for Rgba {
  fn to_hex(&self) -> Hex {
    self.to_rgb().to_hex()
  }
  fn to_rgb(&self) -> Rgb {
    Rgb {
      r: self.r,
      g: self.g,
      b: self.b,
    }
  }

  fn to_rgba(&self) -> Rgba {
    *self
  }

  fn to_hsl(&self) -> Hsl {
    Hsl {
      h: 0,
      s: 0.0,
      l: 0.0,
    }
  }

  fn to_css(&self) -> String {
    format!("rgba({},{},{},{})", self.r, self.g, self.b, self.a)
  }
}

impl SetAlpha<Rgba, Rgba> for Rgba {
  fn set_alpha(&self, a: f32) -> Rgba {
    let mut copy = *self;
    copy.a = normalize_until_one(a);
    copy
  }
}

impl SetRgb<Rgba> for Rgba {
  fn red(&self, val: u8) -> Rgba {
    self.set_rbg("red", val)
  }
  fn green(&self, val: u8) -> Rgba {
    self.set_rbg("green", val)
  }
  fn blue(&self, val: u8) -> Rgba {
    self.set_rbg("blue", val)
  }
}

//
//
//  Hsl
//
//

type HslTuple = (u16, f32, f32);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsl {
  pub h: u16,
  pub s: f32,
  pub l: f32,
}

impl Hsl {
  pub fn as_tuple(&self) -> (u16, f32, f32) {
    (self.h, self.s, self.l)
  }

  pub fn from_tuple(t: HslTuple) -> Hsl {
    Hsl {
      h: normalize_hue(t.0),
      s: normalize_until_one(t.1),
      l: normalize_until_one(t.2),
    }
  }

  fn set_sl(&self, key: &str, val: f32) -> Hsl {
    let mut tuple = self.as_tuple();
    match key {
      "saturation" => {
        tuple.1 = val;
      }
      "lightness" => {
        tuple.2 = val;
      }
      _ => {}
    }
    Hsl::from_tuple(tuple)
  }
}

impl SetHsl<Hsl> for Hsl {
  fn hue(&self, val: u16) -> Hsl {
    let mut tuple = self.as_tuple();
    tuple.0 = normalize_hue(val);
    Hsl::from_tuple(tuple)
  }

  fn saturation(&self, val: f32) -> Hsl {
    self.set_sl("saturation", val)
  }

  fn lightness(&self, val: f32) -> Hsl {
    self.set_sl("lightness", val)
  }
}

//
//
//  Hsla //TODO
//
//
