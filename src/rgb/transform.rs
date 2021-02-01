use consts::RGB_UNIT_MAX;

use crate::{ColorTransform, consts, SaturationInSpace};

use super::{grayscale, Hsl, Rgb};

impl ColorTransform for Rgb {
  /// Lighten or darken color. amt is a percent with negative values - `-100..100`
  /// # Example
  /// ```
  /// use colorsys::{Rgb,ColorTransform, ColorTuple};
  /// let tuple = (30.0, 108.0, 77.0);
  /// let mut rgb = Rgb::from(&tuple);
  ///
  /// rgb.lighten(20.0);
  /// assert_eq!(rgb.to_css_string(), "rgb(52,188,134)" );
  ///
  /// rgb.lighten(-20.0);
  /// assert_eq!(rgb.to_css_string(), "rgb(30,108,77)" );
  ///
  /// rgb.lighten(-20.0);
  /// assert_eq!(rgb.to_css_string(), "rgb(8,28,20)" );
  ///
  /// rgb.lighten(301.123);
  /// assert_eq!(rgb.to_css_string(), "rgb(255,255,255)" );
  /// ```
  fn lighten(&mut self, amt: f64) {
    let mut hsl: Hsl = self.into();
    hsl.lighten(amt);
    let lightened_rgb: Rgb = hsl.as_ref().into();
    self._apply_tuple(&lightened_rgb.into());
  }

  fn saturate(&mut self, sat: SaturationInSpace) {
    match sat {
      SaturationInSpace::Hsl(amt) => {
        let mut hsl: Hsl = self.into();
        hsl.set_saturation(hsl.saturation() + amt);
        let new_rgb = Rgb::from(hsl);
        self._apply_tuple(&new_rgb.into());
      }
      SaturationInSpace::Hsv(amt) => {
        unimplemented!("{}", amt);
      }
    }
  }

  fn adjust_hue(&mut self, hue: f64) {
    let mut hsl: Hsl = self.into();
    hsl.adjust_hue(hue);
    self._apply_tuple(&Rgb::from(hsl).into());
  }

  fn grayscale_simple(&mut self) {
    grayscale::rgb_grayscale(
      self,
      grayscale::GrayScaleMethod::AverageProminent,
    );
  }

  fn invert(&mut self) {
    self.units.list[0].value = RGB_UNIT_MAX - self.units[0];
    self.units.list[1].value = RGB_UNIT_MAX - self.units[1];
    self.units.list[2].value = RGB_UNIT_MAX - self.units[2];
  }
}

#[cfg(test)]
mod test {
  use crate::{Rgb, ColorTransform};

  #[test]
  fn lighten_darken_test() {
    use crate::ColorTuple;
    use crate::common::f64_round;

    pub fn as_rounded_rgb_tuple(t: &ColorTuple) -> (u16, u16, u16) {
      let (r, g, b) = *t;
      (f64_round(r) as u16, f64_round(g) as u16, f64_round(b) as u16)
    }

    let asserts = [
        #[cfg(feature = "std")]
      ((30.0, 108.0, 77.0), 20.0, (52, 188, 134)),
      ((30.0, 108.0, 77.0), 90.0, (255, 255, 255)),
        #[cfg(feature = "std")]
      ((30.0, 108.0, 77.0), -20.0, (8, 28, 20)),
        #[cfg(feature = "std")]
      ((0.0, 0.0, 0.0), 50.0, (128, 128, 128)),
        #[cfg(not(feature = "std"))]
      ((0.0, 0.0, 0.0), 50.0, (127, 127, 127)),
      ((0.0, 0.0, 0.0), -50.0, (0, 0, 0)),
      ((0.0, 0.0, 0.0), 300.5, (255, 255, 255)),
    ];

    for a in asserts.iter() {
      let (origin, amt, result) = *a;
      let mut rgb = Rgb::from(&origin);
      rgb.lighten(amt);
      assert_eq!(as_rounded_rgb_tuple(&rgb.into()), result);
    }
  }
}
