use core::ops::{Add, AddAssign, Sub, SubAssign};

pub use ratio::CmykRatio;

use crate::consts::PERCENT_MAX;
use crate::Rgb;
use crate::units::{Alpha, GetColorUnits, Unit, Units};

mod ratio;
mod from;


/// The CMYK color model.
///
/// Has cyan, magenta, yellow, key (0.0..100.0) and optional `alpha` channel (0.0..1.0).
///
/// # Example
/// ```
/// use colorsys::Cmyk;
///
/// let mut cmyk = Cmyk::new(33.1, 999.9, 11.0, 0.0, None);
///
/// assert_eq!(cmyk.cyan(), 33.1);
/// assert_eq!(cmyk.magenta(), 100.0);
///
/// cmyk.set_yellow(73.0);
/// assert_eq!(cmyk.yellow(), 73.0);
///
/// let doubled_cmyk = &cmyk + &cmyk;
///
/// let units: [f64;4] = doubled_cmyk.into();
/// assert_eq!(units, [66.2,100.0,100.0,0.0]);
///
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Cmyk {
  pub(crate) units: Units,
}
ops_def!(Cmyk);


pub(crate) fn new_cmyk_units(c: f64, m: f64, y: f64, k: f64) -> Units {
  let p = |v: f64| Unit::new_percent(v);
  let ul = [p(c), p(m), p(y), p(k)];
  Units { len: 4, list: ul, alpha: Alpha::default() }
}

impl Cmyk {
  pub fn new(c: f64, m: f64, y: f64, k: f64, a: Option<f64>) -> Self {
    let mut u = new_cmyk_units(c, m, y, k);
    u.alpha.set_opt(a);
    u.restrict();
    Cmyk::from_units(u)
  }

  pub fn cyan(&self) -> f64 { self.units[0] }
  pub fn magenta(&self) -> f64 { self.units[1] }
  pub fn yellow(&self) -> f64 { self.units[2] }
  pub fn key(&self) -> f64 { self.units[3] }

  pub fn set_cyan(&mut self, c: f64) { self.units.list[0].set(c); }
  pub fn set_magenta(&mut self, m: f64) { self.units.list[1].set(m); }
  pub fn set_yellow(&mut self, y: f64) { self.units.list[2].set(y); }
  pub fn set_key(&mut self, k: f64) { self.units.list[3].set(k); }

  /// Returns same color in RGB color model
  /// # Example
  /// ```
  /// use colorsys::{Cmyk, Rgb, ApproxEq};
  ///
  /// let cmyk = Cmyk::from(&[0.0,0.0,100.0,0.0]);
  /// let rgb_from_cmyk = cmyk.as_rgb();
  /// assert!(rgb_from_cmyk.approx_eq(&Rgb::from([255,255,0])));
  ///
  /// ```
  pub fn as_rgb(&self) -> Rgb {
    self.into()
  }
  pub fn as_ratio(&self) -> CmykRatio {
    self.into()
  }

  pub(crate) fn from_units(u: Units) -> Cmyk {
    Cmyk { units: u }
  }
}


impl GetColorUnits for Cmyk {
  fn get_units(&self) -> &Units { &self.units }
  fn get_units_mut(&mut self) -> &mut Units { &mut self.units }
}


impl AsRef<Cmyk> for Cmyk {
  fn as_ref(&self) -> &Cmyk { &self }
}

impl Default for Cmyk {
  fn default() -> Cmyk {
    Cmyk::from_units(new_cmyk_units(0.0, 0.0, 0.0, PERCENT_MAX))
  }
}



