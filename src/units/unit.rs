use core::fmt;
use core::ops::{Add, Sub};

use crate::consts::{ALL_MIN, HUE_MAX, PERCENT_MAX, RATIO_MAX, RGB_UNIT_MAX};

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Unit {
  pub(crate) value: f64,
  kind: UnitType,
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum UnitType {
  Rgb,
  Hue,
  Percent,
  Ratio,
}

impl UnitType {
  fn get_max_value(&self) -> f64 {
    match &self {
      UnitType::Rgb => RGB_UNIT_MAX,
      UnitType::Hue => HUE_MAX,
      UnitType::Percent => PERCENT_MAX,
      UnitType::Ratio => RATIO_MAX,
    }
  }
}

impl Default for Unit {
  fn default() -> Self {
    Unit::new_ratio(0.0)
  }
}


impl fmt::Debug for Unit {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("ColorUnit").field("value", &self.value).finish()
  }
}

impl PartialEq for Unit {
  fn eq(&self, other: &Self) -> bool {
    self.value.eq(&other.value)
  }
}


impl Unit {
  fn new(value: f64, kind: UnitType) -> Self {
    Unit { value, kind }
  }
  fn new_checked(value: f64, kind: UnitType) -> Self {
    let mut u = Unit::new(value, kind);
    u.restrict();
    u
  }

  pub(crate) fn set(&mut self, v: f64) {
    self.value = self.get_restricted(v);
  }

  pub(crate) fn new_rgb(v: f64) -> Self {
    Unit::new(v, UnitType::Rgb)
  }
  pub(crate) fn new_hue(v: f64) -> Self {
    Unit::new(v, UnitType::Hue)
  }
  pub(crate) fn new_percent(v: f64) -> Self {
    Unit::new(v, UnitType::Percent)
  }
  pub(crate) fn new_ratio(v: f64) -> Self {
    Unit::new(v, UnitType::Ratio)
  }

  fn get_restricted(&self, val: f64) -> f64 {
    if val < ALL_MIN {
      return ALL_MIN;
    }
    let max = self.kind.get_max_value();
    if val > max {
      return max;
    }
    val
  }

  pub(crate) fn restrict(&mut self) {
    self.value = self.get_restricted(self.value);
  }

  pub(crate) fn turn_into_ratio(&mut self) {
    self.value /= self.kind.get_max_value();
    self.kind = UnitType::Ratio;
  }

  pub(crate) fn turn_into_percent(&mut self) {
    self.kind = UnitType::Percent;
    self.value *= self.kind.get_max_value();
  }

  pub(crate) fn increase(&mut self, v: f64) {
    self.set(self.value + v)
  }
}


impl Add<Self> for Unit {
  type Output = Unit;
  fn add(self, rhs: Self) -> Self::Output {
    Unit::new_checked(self.value + rhs.value, self.kind)
  }
}


impl Sub<Self> for Unit {
  type Output = Unit;
  fn sub(self, rhs: Self) -> Self::Output {
    Unit::new_checked(self.value - rhs.value, self.kind)
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn test() {}
}
