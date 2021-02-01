use crate::{ColorTransform, SaturationInSpace};
use crate::consts::{ALL_MIN, HUE_MAX};
use crate::normalize::bound_hue;

use super::Hsl;

impl ColorTransform for Hsl {
  fn lighten(&mut self, amt: f64) {
    self.units.list[2].increase(amt);
  }

  fn saturate(&mut self, sat: SaturationInSpace) {
    match sat {
      SaturationInSpace::Hsl(s) => self.units.list[1].increase(s),
      SaturationInSpace::Hsv(s) => {
        unimplemented!("{}", s);
      }
    }
  }

  fn adjust_hue(&mut self, hue: f64) {
    let h = bound_hue(self.units[0] + hue);
    self.units.list[0].set(h);
  }

  fn grayscale_simple(&mut self) {
    self.units.list[0].value = ALL_MIN;
    self.units.list[1].value = ALL_MIN;
  }
  fn invert(&mut self) {
    let h = (self.units[0] + HUE_MAX * 0.5) % HUE_MAX;
    self.units.list[0].set(h);
  }
}
