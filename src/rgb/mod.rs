#[cfg(not(feature = "std"))]
use alloc::string::String;

pub use grayscale::GrayScaleMethod;
use grayscale::rgb_grayscale;
pub use ratio::RgbRatio;

use crate::{ColorAlpha, ColorTuple, ColorTupleA, converters, Hsl, ColorUnitsIter};
use crate::common::{tuple_to_string};
use crate::err::ParseError;
use crate::units::{Alpha, GetColorUnits, Unit, Units};

#[cfg(test)]
mod tests;

mod from;
mod from_str;
mod grayscale;
mod ops;
mod ratio;
mod transform;

/// The RGB color model.
///
/// Has red, green, blue and optional `alpha` channel fields.
/// Red, green, blue values are stored between 0.0 and 255.0, alpha is between 0.0 and 1.0.
/// If inputted or received values are exceeds the allowed value, or is less than zero
/// it will be equalize to limit.
///
/// # Example
/// ```
/// use colorsys::{Rgb, Hsl, prelude::*};
/// let mut rgb1 = Rgb::from((100.0, 255.0, 17.0));
/// // Rgb { r: 100.0, g: 255.0, b: 17.0, a: None }
///
/// let green = rgb1.green();
/// // 255.0
///
/// rgb1.set_red(108.3);
/// // Rgb { r: 108.3, g: 255.0, b: 17.0, .. }
///
/// let mut hsl: Hsl = rgb1.into();
/// // ~Hsl { h: 96.98, s: 100.0, l: 53.333, .. }
///
/// hsl.saturate( SaturationInSpace::Hsl(-57.901) );
/// // ~Hsl { h: 96.98, s: 42.099, l: 53.333, .. }
///
/// let mut rgb2 = Rgb::from(&hsl);
/// // ~Rgb { r: 124.34, g: 186.1, b: 85.9, .. }
///
/// let rgb2tuple: (f64,f64,f64) = rgb2.as_ref().into();
/// // (124.34, 186.1,85.9)
///
/// rgb2 += Rgb::from_hex_str("#35f15b").unwrap();;
/// // ~Rgb { r: 177.33, g: 255.0, b: 176.902, .. }
///
/// rgb2.set_green(-150.0);
/// assert_eq!(rgb2.green(), 0.0);
///
/// rgb2.lighten(-13.123);
/// // ~Rgb { r: 110.41, g: 0.0, b: 110.1, .. }
///
/// rgb2.grayscale_simple();
/// // ~Rgb { r: 55.2, g: 55.2, b: 55.2, .. }
///
/// let css_string = rgb2.to_css_string();
/// assert_eq!(css_string, "rgb(55,55,55)");
/// ```
///
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgb {
  pub(crate) units: Units,
}

iter_def!(Rgb);

pub(crate) fn new_rgb_units(r: f64, g: f64, b: f64) -> Units {
  let ul = [Unit::new_rgb(r), Unit::new_rgb(g), Unit::new_rgb(b), Unit::default()];
  Units { len: 3, list: ul, alpha: Alpha::default() }
}

impl Rgb {
  fn _apply_tuple(&mut self, t: &ColorTuple) {
    self.units.list[0].value = t.0;
    self.units.list[1].value = t.1;
    self.units.list[2].value = t.2;
  }

  pub(crate) fn from_units(u: Units) -> Self { Rgb { units: u } }

  pub fn new(r: f64, g: f64, b: f64, a: Option<f64>) -> Rgb {
    let mut units = new_rgb_units(r, g, b);
    units.alpha.set_opt(a);
    units.restrict();
    Rgb { units }
  }

  pub fn from_hex_str(s: &str) -> Result<Rgb, ParseError> {
    let (tuple, alpha) = converters::hex_to_rgb(s)?;
    let mut rgb = Rgb::from(&tuple);
    if let Some(a) = alpha {
      rgb.set_alpha(a);
    }
    Ok(rgb)
  }

  pub fn to_hex_string(&self) -> String {
    converters::rgb_to_hex(&self.into())
  }

  pub fn to_hexa_string(&self) -> String {
    converters::rgb_to_hexa(&self.into())
  }

  pub fn red(&self) -> f64 { self.units[0] }
  pub fn green(&self) -> f64 {
    self.units[1]
  }
  pub fn blue(&self) -> f64 {
    self.units[2]
  }

  #[deprecated(since = "0.7.0", note = "Please use `red` instead")]
  pub fn get_red(&self) -> f64 { self.red() }
  #[deprecated(since = "0.7.0", note = "Please use `green` instead")]
  pub fn get_green(&self) -> f64 {
    self.green()
  }
  #[deprecated(since = "0.7.0", note = "Please use `blue` instead")]
  pub fn get_blue(&self) -> f64 {
    self.blue()
  }

  pub fn set_red(&mut self, val: f64) { self.units.list[0].set(val); }
  pub fn set_green(&mut self, val: f64) { self.units.list[1].set(val); }
  pub fn set_blue(&mut self, val: f64) { self.units.list[2].set(val); }

  /// Returns a String that can be used in CSS.
  /// # Example
  /// ```
  /// use colorsys::{Rgb};
  ///
  /// let rgb = Rgb::from([55.0,31.1, 201.9]);
  /// assert_eq!(rgb.to_css_string(), "rgb(55,31,202)");
  /// ```
  pub fn to_css_string(&self) -> String {
    let t: ColorTupleA = self.into();
    tuple_to_string(&t, "rgb")
  }

  pub fn grayscale(&mut self, method: GrayScaleMethod) {
    rgb_grayscale(self, method);
  }

  /// Returns an iterator over three color units and the possibly alpha value.
  pub fn iter(&self) -> ColorUnitsIter {
    ColorUnitsIter::from_units(&self.units)
  }

  /// Returns an RGB representation with values converted to floar from 0.0 to 1.0
  pub fn as_ratio(&self) -> RgbRatio {
    RgbRatio::from_units(self.units.as_ratio())
  }
}

//
//
//
// Default
//
impl Default for Rgb {
  fn default() -> Rgb {
    Rgb::from_units(new_rgb_units(0.0, 0.0, 0.0))
  }
}

//
//
//
// AsRef<Rgb>
//
impl AsRef<Rgb> for Rgb {
  fn as_ref(&self) -> &Rgb {
    self
  }
}

impl GetColorUnits for Rgb {
  fn get_units(&self) -> &Units {
    &self.units
  }
  fn get_units_mut(&mut self) -> &mut Units {
    &mut self.units
  }
}

//
//
//
// FromStr
//
impl core::str::FromStr for Rgb {
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
