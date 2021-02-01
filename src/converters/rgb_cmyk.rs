use crate::{Cmyk, Rgb};
use crate::cmyk::CmykRatio;
use crate::consts::{RATIO_MAX, RGB_UNIT_MAX};

pub(crate) fn rgb_to_cmyk(rgb: &Rgb) -> Cmyk {
  let rgb_r = rgb.units.as_ratio();
  let [ r, g, b ]: [f64; 3] = (&rgb_r).into();
  let k = RATIO_MAX - rgb_r.max().0;
  let x = RATIO_MAX - k;

  let c = (x - r) / x;
  let m = (x - g) / x;
  let y = (x - b) / x;

  CmykRatio::new(c, m, y, k, rgb.units.alpha.get_f64()).into()
}

pub(crate) fn cmyk_to_rgb(cmyk: &Cmyk) -> Rgb {
  let [ c, m, y, k ]: [f64;4] = cmyk.units.as_ratio().into();
  let x = RGB_UNIT_MAX * (1.0 - k);

  Rgb::new((1.0 - c) * x, (1.0 - m) * x, (1.0 - y) * x, cmyk.units.alpha.get())
}

#[allow(clippy::float_cmp)]
#[cfg(test)]
mod test {
  use crate::{Cmyk, Rgb};
  use crate::converters::{cmyk_to_rgb, rgb_to_cmyk};

  #[test]
  fn cmyk_to_rbg_test() {
    let cmyk = Cmyk::new(70.0, 23.0, 11.0, 55.0, None);
    let rgb: Rgb = cmyk_to_rgb(&cmyk);
    assert_eq!(rgb.units[0].round(), 34.0);
    assert_eq!(rgb.units[1].round(), 88.0);
    assert_eq!(rgb.units[2].round(), 102.0);
  }

  #[test]
  fn rbg_to_cmyk_test() {
    // let assert_data = [
    //   ([35, 75, 89], [61, 16, 0, 65]),
    //   ([0, 0, 255], [100, 100, 0, 0]),
    //   ([0, 0, 0], [0, 0, 0, 100]),
    //   ([255, 255, 255], [0, 0, 0, 0]),
    // ];

    let rgb = Rgb::new(230.0, 19.0, 70.0, None);
    let cmyk: Cmyk = rgb_to_cmyk(&rgb);
    assert_eq!(cmyk.cyan().round(), 0.0);
    assert_eq!(cmyk.magenta().round(), 92.0);
    assert_eq!(cmyk.yellow().round(), 70.0);
    assert_eq!(cmyk.key().round(), 10.0);
  }
}
