use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{ApproxEq, DEFAULT_APPROX_EQ_PRECISION, Hsl, Rgb};

ops_def!(Rgb);

impl ApproxEq<Hsl> for Rgb {
  fn approx_eq(&self, hsl: &Hsl) -> bool {
    self.approx_eq_clarify(hsl, DEFAULT_APPROX_EQ_PRECISION)
  }

  fn approx_eq_clarify(&self, hsl: &Hsl, precision: f64) -> bool {
    let rgb: Rgb = hsl.into();
    self.units.approx_eq_clarify(&rgb.units, precision)
  }
}

#[cfg(test)]
mod test {
  use crate::{ApproxEq, ColorTupleA, Hsl, Rgb};

  #[test]
  fn rgb_add() {
    let rgb1 = Rgb::default();
    let rgb2 = Rgb::from_hex_str("ffcc00").unwrap();
    let rgb3: ColorTupleA = (rgb1 + rgb2).into();
    assert_eq!(Into::<ColorTupleA>::into(rgb3), (255.0, 204.0, 0.0, 1.0));

    let rgb1 = Rgb::new(200.0, 200.0, 200.0, Some(0.3));
    let rgb2 = Rgb::new(200.0, 200.0, 200.0, None);
    let rgb3: ColorTupleA = (rgb1 + rgb2).into();
    assert_eq!(rgb3, (255.0, 255.0, 255.0, 0.3));
  }

  #[test]
  fn rgb_eq() {
    let rgb1 = Rgb::default();
    let mut rgb2 = Rgb::default();
    let rgb3 = Rgb::from((12.12534, 123.21321, 12.002_310_123));
    let hsl: Hsl = rgb3.as_ref().into();
    let rgb4 = Rgb::from(&hsl);
    rgb2 += rgb3.clone();
    rgb2 -= rgb3.clone();

    assert_eq!(rgb1, rgb2);
    assert!(rgb3.approx_eq(&rgb4));
    assert!(rgb3.approx_eq_clarify(&hsl, 0.000_000_000_001));
  }
}
