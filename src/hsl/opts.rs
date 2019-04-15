use super::Hsl;

// use crate::common::simple_rand;

#[allow(unused_imports)]
use crate::{common, normalize::normalize_opt_ratio, ColorAlpha, ColorTuple, ColorTupleA, Rgb};
use common::{approx::*, ops};

use std::ops::{Add, AddAssign, Sub, SubAssign};

fn add_sub(hsl1: &Hsl, hsl2: &Hsl, is_add: bool) -> Hsl {
  type TA = (ColorTuple, Option<f64>);
  let ta1: TA = (hsl1.into(), hsl1.a);
  let ta2: TA = (hsl2.into(), hsl2.a);
  let (t, a) = ops::add_sub_tuples_a(&ta1, &ta2, is_add);
  let mut hsl = Hsl::from(t);
  hsl.a = normalize_opt_ratio(a);
  hsl
}

fn add(hsl1: &Hsl, hsl2: &Hsl) -> Hsl {
  add_sub(hsl1, hsl2, true)
}

fn sub(hsl1: &Hsl, hsl2: &Hsl) -> Hsl {
  add_sub(hsl1, hsl2, false)
}

impl<'a> Add for &'a Hsl {
  type Output = Hsl;
  fn add(self, rhs: &Hsl) -> Hsl {
    add(self, rhs)
  }
}

impl<'a> Add for &'a mut Hsl {
  type Output = Hsl;
  fn add(self, rhs: &'a mut Hsl) -> Hsl {
    add(self, rhs)
  }
}

impl Add for Hsl {
  type Output = Hsl;
  fn add(self, rhs: Self) -> Self {
    add(&self, &rhs)
  }
}

impl AddAssign for Hsl {
  fn add_assign(&mut self, rhs: Self) {
    *self = add(self, &rhs);
  }
}

impl Sub for Hsl {
  type Output = Hsl;
  fn sub(self, rhs: Self) -> Self {
    sub(&self, &rhs)
  }
}

impl<'a> Sub for &'a Hsl {
  type Output = Hsl;
  fn sub(self, rhs: Self) -> Hsl {
    sub(self, rhs)
  }
}

impl<'a> Sub for &'a mut Hsl {
  type Output = Hsl;
  fn sub(self, rhs: Self) -> Hsl {
    sub(self, rhs)
  }
}

impl SubAssign for Hsl {
  fn sub_assign(&mut self, rhs: Self) {
    *self = sub(self, &rhs);
  }
}

impl ApproxEq<Hsl> for Hsl {
  fn approx_eq(&self, other: &Hsl) -> bool {
    let t1: ColorTuple = self.into();
    let t2: ColorTuple = other.into();
    approx_tuple_def(&t1, &t2) && approx_def(self.get_alpha(), other.get_alpha())
  }
  fn approx_eq_clarify(&self, other: &Hsl, precision: f64) -> bool {
    let t1: ColorTuple = self.into();
    let t2: ColorTuple = other.into();
    approx_tuple(&t1, &t2, precision) && approx(self.get_alpha(), other.get_alpha(), precision)
  }
}

impl ApproxEq<Rgb> for Hsl {
  fn approx_eq(&self, rgb: &Rgb) -> bool {
    self.approx_eq_clarify(rgb, DEFAULT_APPROX_EQ_PRECISION)
  }
  fn approx_eq_clarify(&self, rgb: &Rgb, precision: f64) -> bool {
    let t1: ColorTuple = self.into();
    let t2: ColorTuple = Hsl::from(rgb).into();
    approx_tuple(&t1, &t2, precision) && approx(self.get_alpha(), rgb.get_alpha(), precision)
  }
}

#[test]
fn hsl_add() {}

#[test]
fn hsl_eq() {}
