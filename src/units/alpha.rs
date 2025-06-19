use super::unit::Unit;

use crate::common::approx::approx;
use crate::consts::RATIO_MAX;
use crate::{ApproxEq, DEFAULT_APPROX_EQ_PRECISION};

#[derive(Clone, PartialEq, Debug, Default)]
pub(crate) struct Alpha {
  value: Option<Unit>,
}

impl Alpha {
  pub(crate) fn set(&mut self, a: f64) {
    if let Some(u) = &mut self.value {
      u.set(a);
    } else {
      self.value = Some(Unit::new_ratio(a));
    }
    if self.value.unwrap().value.eq(&RATIO_MAX) {
      self.value = None;
    }
  }
  pub(crate) fn set_opt(&mut self, av: Option<f64>) {
    if let Some(a) = av {
      self.set(a);
    } else {
      self.value = None;
    }
  }

  pub(crate) fn get(&self) -> Option<f64> {
    self.value.map(|u| u.value)
  }
  pub(crate) fn get_f64(&self) -> f64 {
    self.get().unwrap_or(RATIO_MAX)
  }

  pub(crate) fn opacify(&mut self, v: f64) {
    self.set(self.get_f64() + v);
  }
}

impl ApproxEq<Alpha> for Alpha {
  fn approx_eq(&self, other: &Alpha) -> bool {
    self.approx_eq_clarify(other, DEFAULT_APPROX_EQ_PRECISION)
  }

  fn approx_eq_clarify(&self, other: &Alpha, precision: f64) -> bool {
    if let Some(su) = &self.value {
      if let Some(ou) = &other.value {
        return approx(su.value, ou.value, precision);
      }
    } else {
      return other.value.is_none();
    }
    false
  }
}
