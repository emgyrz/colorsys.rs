#[cfg(not(feature = "std"))]
use alloc::string::String;

pub use ratio::HslRatio;

use crate::common::{Hs, hsl_hsv_from_str, tuple_to_string};
use crate::units::{Alpha, GetColorUnits, Unit, Units};
use crate::{ColorAlpha, ColorTupleA, ColorUnitsIter, ParseError, Rgb};

#[cfg(test)]
mod tests;

mod from;
mod ops;
mod ratio;
mod transform;

/// The HSL or HSI (hue, saturation, lightness (intensity)) color model
///
/// Ranges:
/// * hue: 0.0 - 360.0
/// * saturation: 0.0 - 100.0
/// * saturation: 0.0 - 100.0
/// * alpha: 0.0 - 1.0
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hsl {
  pub(crate) units: Units,
}

iter_def!(Hsl);
pub(crate) fn new_hsl_units(h: f64, s: f64, l: f64) -> Units {
  let ul = [
    Unit::new_hue(h),
    Unit::new_percent(s),
    Unit::new_percent(l),
    Unit::default(),
  ];
  Units { len: 3, list: ul, alpha: Alpha::default() }
}


impl Hsl {
  pub fn new(h: f64, s: f64, l: f64, a: Option<f64>) -> Hsl {
    let mut units = new_hsl_units(h, s, l);
    units.alpha.set_opt(a);
    units.restrict();
    Hsl { units }
  }

  pub(crate) fn from_units(u: Units) -> Self {
    Hsl { units: u }
  }

  pub fn to_css_string(&self) -> String {
    let t: ColorTupleA = self.into();
    tuple_to_string(&t, "hsl")
  }

  pub fn hue(&self) -> f64 {
    self.units[0]
  }
  pub fn saturation(&self) -> f64 {
    self.units[1]
  }
  pub fn lightness(&self) -> f64 {
    self.units[2]
  }

  #[deprecated(since = "0.7.0", note = "Please use `hue` instead")]
  pub fn get_hue(&self) -> f64 {
    self.hue()
  }
  #[deprecated(since = "0.7.0", note = "Please use `saturation` instead")]
  pub fn get_saturation(&self) -> f64 {
    self.saturation()
  }
  #[deprecated(since = "0.7.0", note = "Please use `lightness` instead")]
  pub fn get_lightness(&self) -> f64 {
    self.lightness()
  }

  pub fn set_hue(&mut self, val: f64) {
    self.units.list[0].set(val);
  }
  pub fn set_saturation(&mut self, val: f64) {
    self.units.list[1].set(val);
  }
  pub fn set_lightness(&mut self, val: f64) {
    self.units.list[2].set(val);
  }

  /// Returns an iterator over three color units and the possibly alpha value.
  pub fn iter(&self) -> ColorUnitsIter {
    ColorUnitsIter::from_units(&self.units)
  }

  /// Returns an HSL representation with values converted to floar from 0.0 to 1.0
  pub fn as_ratio(&self) -> HslRatio {
    HslRatio::from_units(self.units.as_ratio())
  }
}

//
//
//
// Default
//
impl Default for Hsl {
  fn default() -> Hsl {
    Hsl::from_units(new_hsl_units(0.0, 0.0, 0.0))
  }
}

//
//
//
// AsRef<Hsl>
//
impl AsRef<Hsl> for Hsl {
  fn as_ref(&self) -> &Hsl {
    self
  }
}

//
//
//
// FromStr
//
impl core::str::FromStr for Hsl {
  type Err = ParseError;
  fn from_str(s: &str) -> Result<Hsl, ParseError> {
    let (tuple, alpha) = hsl_hsv_from_str(s, Hs::Hsl)?;
    let mut hsl = Hsl::from(&tuple);
    if let Some(a) = alpha {
      hsl.set_alpha(a);
    }
    Ok(hsl)
  }
}


impl GetColorUnits for Hsl {
  fn get_units(&self) -> &Units {
    &self.units
  }
  fn get_units_mut(&mut self) -> &mut Units {
    &mut self.units
  }
}
