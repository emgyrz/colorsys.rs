use crate::normalize::normalize_ratio;

///
/// Hsl representation as ratio (from `0.0` to `1.0`).
/// Cannot be modified, added, subtracted, etc. Can be converted to `Hsl` and vice versa.
/// Used for compatibility in various libraries.
///
/// # Example
/// ```rust
/// use colorsys::{ApproxEq, Hsl, HslRatio};
///
/// let origin = Hsl::from((126.0, 43.0, 52.0));
///
/// let ratio_f32: [f32; 3] = origin.as_ratio().into();
/// let ratio_f64: [f64; 4] = origin.as_ratio().into();
/// // ~[0.35, 0.43, 0.52, 1.0]
///
/// let converted_f32: Hsl = HslRatio::from(&ratio_f32).into();
/// let converted_f64: Hsl = HslRatio::from(&ratio_f64).into();
///
/// assert!(origin.approx_eq_clarify(&converted_f32, 0.0001));
/// assert!(origin.approx_eq(&converted_f64));
/// ```
///
#[derive(Clone)]
pub struct HslRatio {
  pub(super) h: f64,
  pub(super) s: f64,
  pub(super) l: f64,
  pub(super) a: f64,
}

impl HslRatio {
  pub fn new(h: f64, s: f64, l: f64, a: f64) -> Self {
    HslRatio {
      h: normalize_ratio(h),
      s: normalize_ratio(s),
      l: normalize_ratio(l),
      a: normalize_ratio(a),
    }
  }

  pub fn h(&self) -> f64 {
    self.h
  }
  pub fn s(&self) -> f64 {
    self.s
  }
  pub fn l(&self) -> f64 {
    self.l
  }
  pub fn a(&self) -> f64 {
    self.a
  }
}

impl AsRef<HslRatio> for HslRatio {
  fn as_ref(&self) -> &HslRatio {
    &self
  }
}

macro_rules! from_for_hsl_ratio {
  ($from_type: ty, $val: ident, $conv: block) => {
    impl From<&$from_type> for HslRatio {
      fn from($val: &$from_type) -> HslRatio {
        ($conv)
      }
    }
    impl From<$from_type> for HslRatio {
      fn from($val: $from_type) -> HslRatio {
        HslRatio::from(&$val)
      }
    }
  };
}

macro_rules! from_for_hsl_ratio_all {
  ($t: ty) => {
    from_for_hsl_ratio!(($t, $t, $t), v, {
      let (h, s, l) = *v;
      HslRatio::new(h as f64, s as f64, l as f64, 1.0)
    });
    from_for_hsl_ratio!(($t, $t, $t, $t), v, {
      let (h, s, l, a) = *v;
      HslRatio::new(h as f64, s as f64, l as f64, a as f64)
    });
    from_for_hsl_ratio!([$t; 3], v, {
      let [h, s, l] = *v;
      HslRatio::new(h as f64, s as f64, l as f64, 1.0)
    });
    from_for_hsl_ratio!([$t; 4], v, {
      let [h, s, l, a] = *v;
      HslRatio::new(h as f64, s as f64, l as f64, a as f64)
    });
  };
}

from_for_hsl_ratio_all!(f32);
from_for_hsl_ratio_all!(f64);

macro_rules! into_for_hsl_ratio_all {
  ($t: ty) => {
    into_for_some!(($t, $t, $t), HslRatio, self, {
      let HslRatio { h, s, l, .. } = *self;
      (h as $t, s as $t, l as $t)
    });
    into_for_some!(($t, $t, $t, $t), HslRatio, self, {
      let HslRatio { h, s, l, a } = *self;
      (h as $t, s as $t, l as $t, a as $t)
    });
    into_for_some!([$t; 3], HslRatio, self, {
      let HslRatio { h, s, l, .. } = *self;
      [h as $t, s as $t, l as $t]
    });
    into_for_some!([$t; 4], HslRatio, self, {
      let HslRatio { h, s, l, a } = *self;
      [h as $t, s as $t, l as $t, a as $t]
    });
  };
}

into_for_hsl_ratio_all!(f32);
into_for_hsl_ratio_all!(f64);
