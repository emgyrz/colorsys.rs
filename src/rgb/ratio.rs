use crate::normalize::normalize_ratio;

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
#[derive(Clone)]
pub struct RgbRatio {
  pub(super) r: f64,
  pub(super) g: f64,
  pub(super) b: f64,
  pub(super) a: f64,
}

impl RgbRatio {
  pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
    RgbRatio {
      r: normalize_ratio(r),
      g: normalize_ratio(g),
      b: normalize_ratio(b),
      a: normalize_ratio(a),
    }
  }

  pub fn r(&self) -> f64 {
    self.r
  }
  pub fn g(&self) -> f64 {
    self.g
  }
  pub fn b(&self) -> f64 {
    self.b
  }
  pub fn a(&self) -> f64 {
    self.a
  }
}

impl AsRef<RgbRatio> for RgbRatio {
  fn as_ref(&self) -> &RgbRatio {
    &self
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
      let RgbRatio { r, g, b, .. } = *self;
      (r as $t, g as $t, b as $t)
    });
    into_for_some!(($t, $t, $t, $t), RgbRatio, self, {
      let RgbRatio { r, g, b, a } = *self;
      (r as $t, g as $t, b as $t, a as $t)
    });
    into_for_some!([$t; 3], RgbRatio, self, {
      let RgbRatio { r, g, b, .. } = *self;
      [r as $t, g as $t, b as $t]
    });
    into_for_some!([$t; 4], RgbRatio, self, {
      let RgbRatio { r, g, b, a } = *self;
      [r as $t, g as $t, b as $t, a as $t]
    });
  };
}

into_for_rgb_ratio_all!(f32);
into_for_rgb_ratio_all!(f64);
