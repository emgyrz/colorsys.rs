use core::fmt;
use core::ops::{Add, Sub};

use crate::consts::{ALL_MIN, HUE_MAX, PERCENT_MAX, RATIO_MAX, RGB_UNIT_MAX};

#[derive(Clone, Copy)]
pub struct Unit {
  pub(crate) value: f64,
  highest: &'static f64,
}


impl Default for Unit {
  fn default() -> Self {
    Unit::new_ratio(0.0)
  }
}


impl fmt::Debug for Unit {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("ColorUnit")
      .field("value", &self.value)
      .finish()
  }
}

impl PartialEq for Unit {
  fn eq(&self, other: &Self) -> bool {
    self.value.eq(&other.value)
  }
}


impl Unit {
  fn new(value: f64, highest: &'static f64) -> Self {
    Unit {
      value,
      highest,
    }
  }
  fn new_checked(value: f64, highest: &'static f64) -> Self {
    let mut u = Unit::new(value, highest);
    u.restrict();
    u
  }

  pub(crate) fn set(&mut self, v: f64) {
    self.value = self.get_restricted(v);
  }

  pub(crate) fn new_rgb(v: f64) -> Self {
    Unit::new(v, &RGB_UNIT_MAX)
  }
  pub(crate) fn new_hue(v: f64) -> Self {
    Unit::new(v, &HUE_MAX)
  }
  pub(crate) fn new_percent(v: f64) -> Self {
    Unit::new(v, &PERCENT_MAX)
  }
  pub(crate) fn new_ratio(v: f64) -> Self {
    Unit::new(v, &RATIO_MAX)
  }

  fn get_restricted(&self, val: f64) -> f64 {
    if val < ALL_MIN {
      return ALL_MIN;
    } else if &val > self.highest {
      return *self.highest;
    }
    val
  }

  pub(crate) fn restrict(&mut self) {
    self.value = self.get_restricted(self.value);
  }

  pub(crate) fn turn_into_ratio(&mut self) {
    self.value /= self.highest;
    self.highest = &RATIO_MAX;
  }

  pub(crate) fn turn_into_whole(&mut self, highest: &'static f64) {
    self.highest = highest;
    self.value *= self.highest;
  }

  pub(crate) fn increase(&mut self, v: f64) {
    self.set(self.value + v)
  }
}


impl Add<Self> for Unit {
  type Output = Unit;
  fn add(self, rhs: Self) -> Self::Output {
    Unit::new_checked(self.value + rhs.value, self.highest)
  }
}


impl Sub<Self> for Unit {
  type Output = Unit;
  fn sub(self, rhs: Self) -> Self::Output {
    Unit::new_checked(self.value - rhs.value, self.highest)
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn test() {}
}
