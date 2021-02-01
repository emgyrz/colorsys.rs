use crate::units::GetColorUnits;

/// Methods to work with alpha channel in color.
pub trait ColorAlpha {
  /// Returns alpha channel. If it not set will returns 1.0
  fn alpha(&self) -> f64;
  /// Returns alpha channel. If it not set will returns 1.0
  #[deprecated(since = "0.7.0", note = "Please use `alpha` instead")]
  fn get_alpha(&self) -> f64;

  /// Sets alpha channel
  /// ```
  /// use colorsys::{Hsl,ColorAlpha};
  /// let mut hsl = Hsl::default(); // Hsl { a: None, .. }
  /// hsl.set_alpha(0.45); // Hsl { a: 0.45, .. }
  /// hsl.set_alpha(123.015); // Hsl { a: 1.0, .. }
  /// hsl.set_alpha(-123.3); // Hsl { a: 0.0, .. }
  /// ```
  fn set_alpha(&mut self, val: f64);

  /// Increase/decrease color alpha channel with specified value. Value can be negative.
  /// # Example
  /// ```
  /// use colorsys::{Hsl,ColorAlpha};
  /// let mut hsl = Hsl::default(); // Hsl { a: None, .. }
  /// hsl.opacify(-0.3); // Hsl { a: 0.7, .. }
  /// hsl.opacify(0.015); // Hsl { a: 0.715, .. }
  /// ```
  fn opacify(&mut self, val: f64);
}



impl<T> ColorAlpha for T where T: GetColorUnits {
  fn alpha(&self) -> f64 { self.get_units().alpha.get_f64() }
  fn get_alpha(&self) -> f64 {
    self.alpha()
  }
  fn set_alpha(&mut self, val: f64) {
    self.get_units_mut().alpha.set(val);
  }
  fn opacify(&mut self, val: f64) {
    self.get_units_mut().alpha.opacify(val);
  }
}
