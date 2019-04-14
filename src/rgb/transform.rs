use super::{grayscale, Hsl, Rgb};
use crate::{consts, ColorTransform, SaturationInSpace};
use consts::RGB_UNIT_MAX;

impl ColorTransform for Rgb {
  fn lighten(&mut self, amt: f64) {
    let mut hsl: Hsl = self.into();
    hsl.lighten(amt);
    let lightened_rgb: Rgb = hsl.into();
    self._apply_tuple(&lightened_rgb.into());
  }

  fn saturate(&mut self, sat: SaturationInSpace) {
    match sat {
      SaturationInSpace::Hsl(amt) => {
        let mut hsl: Hsl = self.into();
        hsl.set_saturation(hsl.get_saturation() + amt);
        let new_rgb = Rgb::from(hsl);
        self._apply_tuple(&new_rgb.into());
      }
      SaturationInSpace::Hsv(amt) => {
        println!("{}", amt);
        unimplemented!();
      }
    }
  }

  fn adjust_hue(&mut self, hue: f64) {
    let mut hsl: Hsl = self.into();
    hsl.adjust_hue(hue);
    self._apply_tuple(&Rgb::from(hsl).into());
  }

  fn grayscale_simple(&mut self) {
    grayscale::rgb_grayscale(self, grayscale::GrayScaleMethod::AverageProminent);
  }

  fn invert(&mut self) {
    self.r = RGB_UNIT_MAX - self.r;
    self.g = RGB_UNIT_MAX - self.g;
    self.b = RGB_UNIT_MAX - self.b;
  }
}
