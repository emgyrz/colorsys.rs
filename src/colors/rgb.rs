use super::{Hsl, Hsla, Rgba};
use crate::converters::{
  as_rounded_rgb_tuple, rgb_invert, rgb_to_grayscale, rgb_to_grayscale_rec2100,
  rgb_to_grayscale_rec709, rgb_to_hex, rgb_to_hsl,
};
use crate::error::ParseError;
use crate::normalize::{normalize_rgb, normalize_rgb_unit};
use crate::{from_str, Color, ColorTuple, RgbColor};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb {
  r: f32,
  g: f32,
  b: f32,
}

impl Rgb {
  pub fn from(r: f32, g: f32, b: f32) -> Rgb {
    Rgb::from_tuple(&(r, g, b))
  }

  /// Try to parse string as hex color
  /// # Example
  /// ```
  /// use colors_transform::{Rgb,Color};
  ///
  /// assert_eq!(Rgb::from_hex_str("#e76B2c").unwrap(),Rgb::from(231.0,107.0,44.0));
  /// assert_eq!(Rgb::from_hex_str("fc0").unwrap(),Rgb::from_tuple(&(255.0,204.0,0.0)));
  /// assert!(Rgb::from_hex_str("cyan").is_err());
  /// ```
  pub fn from_hex_str(s: &str) -> Result<Rgb, ParseError> {
    match from_str::hex(s) {
      Ok(rgb_tuple) => Ok(Rgb::from_tuple(&rgb_tuple)),
      Err(err) => Err(err),
    }
  }
  /// Returns hexadecimal color string like in css. In lowercase with no reductions
  /// # Example
  /// ```
  /// use colors_transform::{Rgb,Color};
  ///
  /// let rgb1 = Rgb::from_tuple(&(231.0,107.0,44.0));
  /// assert_eq!(rgb1.to_css_hex_string(),"#e76b2c");
  ///
  /// let rgb2 = Rgb::from_hex_str("#0C7").unwrap();
  /// assert_eq!(rgb2.to_css_hex_string(),"#00cc77");
  /// ```
  pub fn to_css_hex_string(&self) -> String {
    let (r, g, b) = rgb_to_hex(&self.as_tuple());
    format!("#{}{}{}", r, g, b)
  }

  /// Convert color to grayscale using the ITU-R BT.709 standard used for HDTV
  pub fn grayscale_rec709(&self) -> Rgb {
    Rgb::from_tuple(&rgb_to_grayscale_rec709(&self.as_tuple()))
  }

  /// Convert color to grayscale using the ITU-R BT.2100 standard for HDR television
  pub fn grayscale_rec2100(&self) -> Rgb {
    Rgb::from_tuple(&rgb_to_grayscale_rec2100(&self.as_tuple()))
  }
}

impl std::str::FromStr for Rgb {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Rgb, ParseError> {
    match from_str::rgb(s) {
      Ok(rgb_tuple) => Ok(Rgb::from_tuple(&rgb_tuple)),
      Err(err) => Err(err),
    }
  }
}

impl Color for Rgb {
  type Tuple = ColorTuple;

  fn new() -> Rgb {
    Rgb { r: 0.0, g: 0.0, b: 0.0 }
  }

  fn get_red(&self) -> f32 {
    self.r
  }
  fn get_green(&self) -> f32 {
    self.g
  }
  fn get_blue(&self) -> f32 {
    self.b
  }
  fn set_red(&self, val: f32) -> Rgb {
    Rgb { r: normalize_rgb_unit(val), g: self.g, b: self.b }
  }
  fn set_green(&self, val: f32) -> Rgb {
    Rgb { r: self.r, g: normalize_rgb_unit(val), b: self.b }
  }
  fn set_blue(&self, val: f32) -> Rgb {
    Rgb { r: self.r, g: self.g, b: normalize_rgb_unit(val) }
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
  fn set_hue(&self, val: f32) -> Rgb {
    self.to_hsl().set_hue(val).to_rgb()
  }
  fn set_saturation(&self, val: f32) -> Rgb {
    self.to_hsl().set_saturation(val).to_rgb()
  }
  fn set_lightness(&self, val: f32) -> Rgb {
    self.to_hsl().set_lightness(val).to_rgb()
  }

  fn to_rgb(&self) -> Rgb {
    *self
  }
  fn to_rgba(&self) -> Rgba {
    let (r, g, b) = self.as_tuple();
    Rgba::from_tuple(&(r, g, b, 1.0))
  }
  fn to_hsl(&self) -> Hsl {
    Hsl::from_tuple(&rgb_to_hsl(&self.as_tuple()))
  }
  fn to_hsla(&self) -> Hsla {
    self.to_hsl().to_hsla()
  }
  /// Returns css string
  /// # Example
  /// ```
  /// use colors_transform::{Rgb,Color};
  ///
  /// let rgb = Rgb::from_tuple(&(225.0,101.7, 21.0));
  /// assert_eq!(rgb.to_css_string(), "rgb(225,102,21)");
  /// ```
  fn to_css_string(&self) -> String {
    let (r, g, b) = as_rounded_rgb_tuple(&self.as_tuple());
    format!("rgb({},{},{})", r, g, b)
  }

  fn from_tuple(t: &ColorTuple) -> Rgb {
    let (r, g, b) = normalize_rgb(&t);
    Rgb { r, g, b }
  }
  fn as_tuple(&self) -> ColorTuple {
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
    let (r, g, b) = self.as_tuple();
    match name {
      RgbColor::Red => self.set_red(r + val),
      RgbColor::Green => self.set_green(g + val),
      RgbColor::Blue => self.set_blue(b + val),
    }
  }

  fn grayscale(&self) -> Rgb {
    Rgb::from_tuple(&rgb_to_grayscale(&self.as_tuple()))
  }

  fn invert(&self) -> Rgb {
    Rgb::from_tuple(&rgb_invert(&self.as_tuple()))
  }
}
