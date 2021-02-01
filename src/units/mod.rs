use core::ops::{Add, Index, Sub};

pub(crate) use alpha::Alpha;
pub(crate) use unit::Unit;

use crate::{ApproxEq, DEFAULT_APPROX_EQ_PRECISION};
use crate::common::approx::approx;

mod unit;
mod alpha;
mod into;
pub mod iter;

pub trait GetColorUnits {
  fn get_units(&self) -> &Units;
  fn get_units_mut(&mut self) -> &mut Units;
}


#[derive(Clone, PartialEq, Debug)]
pub struct Units {
  pub(crate) len: usize,
  pub(crate) list: [Unit; 4],
  pub(crate) alpha: Alpha,
}

impl Units {
  pub(crate) fn restrict(&mut self) {
    for i in 0..self.len {
      self.list[i].restrict();
    }
  }

  fn add_sub(&self, other: &Units, is_add: bool) -> Self {
    let mut new = self.clone();
    for i in 0..self.len {
      let a = new.list[i];
      let b = other.list[i];
      let x = if is_add { a + b } else { a - b };
      new.list[i] = x;
    }
    new
  }

  pub(crate) fn as_ratio(&self) -> Self {
    let mut new = self.clone();
    for i in 0..new.len {
      new.list[i].turn_into_ratio();
    }
    new
  }

  pub(crate) fn max(&self) -> (f64, usize) {
    self.min_max(true)
  }
  pub(crate) fn min(&self) -> (f64, usize) {
    self.min_max(false)
  }

  fn min_max(&self, need_max: bool) -> (f64, usize) {
    let mut result = self.list[0].value;
    let mut ind = 0;
    if self.len != 1 {
      for i in 1..self.len {
        let v = self.list[i].value;
        let is_fit = if need_max { v > result } else { v < result };
        if is_fit {
          result = v;
          ind = i;
        }
      }
    }
    (result, ind)
  }


  pub(crate) fn new_ratios(values: &[f64]) -> Units {
    if values.len() > 4 { panic!("length of units values is more than 4") }
    let mut ul: [Unit; 4] = Default::default();
    for (ind, v) in values.iter().enumerate() {
      ul[ind].set(*v);
    }
    Units { len: values.len(), list: ul, alpha: Alpha::default() }
  }
}


impl Index<usize> for Units {
  type Output = f64;

  fn index(&self, ind: usize) -> &Self::Output {
    &self.list[ind].value
  }
}

impl<'a> Add<Self> for &'a Units {
  type Output = Units;
  fn add(self, rhs: &'a Units) -> Self::Output {
    self.add_sub(rhs, true)
  }
}

impl<'a> Sub<Self> for &'a Units {
  type Output = Units;
  fn sub(self, rhs: &'a Units) -> Self::Output {
    self.add_sub(rhs, false)
  }
}


impl ApproxEq<Units> for Units {
  fn approx_eq(&self, other: &Units) -> bool {
    self.approx_eq_clarify(other, DEFAULT_APPROX_EQ_PRECISION)
  }

  fn approx_eq_clarify(&self, other: &Units, precision: f64) -> bool {
    if !self.alpha.approx_eq_clarify(&other.alpha, precision) {
      return false
    }
    for i in 0..self.len {
      if !approx(self.list[i].value, other.list[i].value, precision) {
        return false
      }
    }
    true
  }
}


#[cfg(test)]
mod test {
  #[test]
  fn test() {}
}
