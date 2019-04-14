use crate::consts::{ALL_MIN, HUE_MAX};
use crate::normalize::bound_hue;
use crate::{ColorTransform, SaturationInSpace};

use super::Hsl;

impl ColorTransform for Hsl {
  fn lighten(&mut self, amt: f64) {
    self.set_lightness(self.l + amt)
  }

  fn saturate(&mut self, sat: SaturationInSpace) {
    match sat {
      SaturationInSpace::Hsl(s) => self.set_saturation(self.s + s),
      SaturationInSpace::Hsv(s) => {
        println!("{}", s);
        unimplemented!();
      }
    }
  }

  fn adjust_hue(&mut self, hue: f64) {
    self.h = bound_hue(self.h + hue);
  }

  fn grayscale_simple(&mut self) {
    self.h = ALL_MIN;
    self.s = ALL_MIN;
  }
  fn invert(&mut self) {
    self.h = (self.h + HUE_MAX * 0.5) % HUE_MAX
  }
}
