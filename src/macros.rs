#![macro_use]

macro_rules! into_for_some {
  ($into: ty, $some: ty, $sel: ident, $conv: block) => {
    impl<'a> Into<$into> for &'a $some {
      fn into($sel) -> $into {
        ($conv)
      }
    }

    impl<'a> Into<$into> for &'a mut $some {
      fn into($sel) -> $into {
        ($conv)
      }
    }

    impl Into<$into> for $some {
      fn into($sel) -> $into {
        $sel.as_ref().into()
      }
    }
  };
}




macro_rules! ops_def {
  ($name: ident) => {

impl<'a> Add for &'a $name {
  type Output = $name;
  fn add(self, rhs: &$name) -> $name { $name::from_units(&self.units + &rhs.units) }
}
impl<'a> Add for &'a mut $name {
  type Output = $name;
  fn add(self, rhs: &'a mut $name) -> $name { $name::from_units(&self.units + &rhs.units) }
}
impl Add for $name {
  type Output = $name;
  fn add(self, rhs: Self) -> Self { $name::from_units(&self.units + &rhs.units) }
}
impl AddAssign for $name {
  fn add_assign(&mut self, rhs: Self) { *self = $name::from_units(&self.units + &rhs.units); }
}


impl Sub for $name {
  type Output = $name;
  fn sub(self, rhs: Self) -> Self { $name::from_units(&self.units - &rhs.units) }
}

impl<'a> Sub for &'a $name {
  type Output = $name;
  fn sub(self, rhs: Self) -> $name { $name::from_units(&self.units - &rhs.units) }
}

impl<'a> Sub for &'a mut $name {
  type Output = $name;
  fn sub(self, rhs: Self) -> $name { $name::from_units(&self.units - &rhs.units) }
}

impl SubAssign for $name {
  fn sub_assign(&mut self, rhs: Self) { *self = $name::from_units(&self.units - &rhs.units); }
}

};
}



macro_rules! iter_def {
  ($name: ident) => {
impl<'a> core::iter::IntoIterator for &'a $name {
  type Item = f64;
  type IntoIter = ColorUnitsIter;
  fn into_iter(self) -> ColorUnitsIter {
    self.iter()
  }
}

impl core::iter::IntoIterator for $name {
  type Item = f64;
  type IntoIter = ColorUnitsIter;
  fn into_iter(self) -> ColorUnitsIter {
    self.iter()
  }
}

};
}
