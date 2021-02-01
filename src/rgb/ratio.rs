use crate::units::{GetColorUnits, Units};

///
/// Rgb representation as ratio (from `0.0` to `1.0`).
/// Cannot be modified, added, subtracted, etc. Can be converted to `Rgb` and vice versa.
/// Used for compatibility in various libraries.
///
/// # Example
/// ```rust
///  use colorsys::{ApproxEq, Rgb, RgbRatio};
///
///  let origin = Rgb::from([134.9, 11.1, 250.55, 1.0]);
///
///  let ratio_f32: [f32; 4] = origin.as_ratio().into();
///  let ratio_f64: [f64; 3] = origin.as_ratio().into();
///  // ~[0.5290196, 0.04352941, 0.982549]
///
///  let converted_f32: Rgb = RgbRatio::from(&ratio_f32).into();
///  let converted_f64: Rgb = RgbRatio::from(&ratio_f64).into();
///
///  assert!(origin.approx_eq_clarify(&converted_f32, 0.0001));
///  assert!(origin.approx_eq(&converted_f64));
///
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub struct RgbRatio {
  pub(crate) units: Units,
}

impl RgbRatio {
  pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
    let mut units = Units::new_ratios(&[r, g, b]);
    units.alpha.set(a);
    RgbRatio { units }
  }

  pub fn r(&self) -> f64 { self.units[0] }
  pub fn g(&self) -> f64 { self.units[1] }
  pub fn b(&self) -> f64 { self.units[2] }
  pub fn a(&self) -> f64 { self.units.alpha.get_f64() }

  pub(crate) fn from_units(u: Units) -> Self { RgbRatio { units: u } }
}

impl AsRef<RgbRatio> for RgbRatio {
  fn as_ref(&self) -> &RgbRatio {
    &self
  }
}


impl GetColorUnits for RgbRatio {
  fn get_units(&self) -> &Units {
    &self.units
  }
  fn get_units_mut(&mut self) -> &mut Units {
    &mut self.units
  }
}

macro_rules! from_for_rgb_ratio {
  ($from_type: ty, $val: ident, $conv: block) => {
    impl From<&$from_type> for RgbRatio {
      fn from($val: &$from_type) -> RgbRatio {
        ($conv)
      }
    }
    impl From<$from_type> for RgbRatio {
      fn from($val: $from_type) -> RgbRatio {
        RgbRatio::from(&$val)
      }
    }
  };
}

macro_rules! from_for_rgb_ratio_all {
  ($t: ty) => {
    from_for_rgb_ratio!(($t, $t, $t), v, {
      let (r, g, b) = *v;
      RgbRatio::new(r as f64, g as f64, b as f64, 1.0)
    });
    from_for_rgb_ratio!(($t, $t, $t, $t), v, {
      let (r, g, b, a) = *v;
      RgbRatio::new(r as f64, g as f64, b as f64, a as f64)
    });
    from_for_rgb_ratio!([$t; 3], v, {
      let [r, g, b] = *v;
      RgbRatio::new(r as f64, g as f64, b as f64, 1.0)
    });
    from_for_rgb_ratio!([$t; 4], v, {
      let [r, g, b, a] = *v;
      RgbRatio::new(r as f64, g as f64, b as f64, a as f64)
    });
  };
}

from_for_rgb_ratio_all!(f32);
from_for_rgb_ratio_all!(f64);

macro_rules! into_for_rgb_ratio_all {
  ($t: ty) => {
    into_for_some!(($t, $t, $t), RgbRatio, self, {
      let u = &self.get_units();
      (u[0] as $t, u[1] as $t, u[2] as $t)
    });
    into_for_some!(($t, $t, $t, $t), RgbRatio, self, {
      let u = &self.get_units();
      (u[0] as $t, u[1] as $t, u[2] as $t, u.alpha.get_f64() as $t)
    });
    into_for_some!([$t; 3], RgbRatio, self, {
      let u = &self.get_units();
      [u[0] as $t, u[1] as $t, u[2] as $t]
    });
    into_for_some!([$t; 4], RgbRatio, self, {
      let u = &self.get_units();
      [u[0] as $t, u[1] as $t, u[2] as $t, u.alpha.get_f64() as $t]
    });
  };
}

into_for_rgb_ratio_all!(f32);
into_for_rgb_ratio_all!(f64);
