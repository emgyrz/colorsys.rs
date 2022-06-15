use crate::Rgb;
use crate::consts::{ALL_MIN, PERCENT_MAX};
use crate::hsl::new_hsl_units;
use crate::units::Units;

pub fn rgb_to_hsl(rgb: &Rgb) -> Units {
  let rgb_r = rgb.units.as_ratio();

  let (max, max_unit) = rgb_r.max();
  let (min, _) = rgb_r.min();
  let max_plus_min = max + min;
  let luminance = (max_plus_min) / 2.0;

  if max.eq(&min) {
    return new_hsl_units(ALL_MIN, ALL_MIN, luminance * PERCENT_MAX);
  }

  let max_min_delta = max - min;
  let saturation = if luminance > 0.5 {
    max_min_delta / (2.0 - max_plus_min)
  } else {
    max_min_delta / (max_plus_min)
  };

  let [red, green, blue]: [f64; 3] = rgb_r.into();

  let hue = match max_unit {
    // red
    0 => {
      let x = if green < blue { 6.0 } else { ALL_MIN };
      (green - blue) / max_min_delta + x
    }
    // green
    1 => (blue - red) / max_min_delta + 2.0,
    // blue
    2 => (red - green) / max_min_delta + 4.0,
    _ => { unreachable!() }
  };

  new_hsl_units(hue * 60.0, saturation * PERCENT_MAX, luminance * PERCENT_MAX)
}

#[test]
fn rgb_to_hsl_tst() {
  use crate::{ColorTuple, Rgb, ApproxEq};
  fn a(x: ColorTuple, y: ColorTuple) -> bool {
    let from_rgb_u = rgb_to_hsl(&Rgb::from(x));
    let hsl_u = new_hsl_units(y.0, y.1, y.2);
    from_rgb_u.approx_eq_clarify(&hsl_u, 0.5)
    // Rgb::from(x).approx_eq_clarify(&Hsl::from(y), 0.5)
    // approx_tuple(&rgb_to_hsl(&Rgb::from(x)), &y, 0.5)
  }
  let asserts = [
    ((255.0, 255.0, 255.0), (0.0, 0.0, 100.0)),
    ((0.0, 0.0, 0.0), (0.0, 0.0, 0.0)),
    ((215.0, 231.0, 236.0), (194.0, 36.0, 88.0)),
    ((108.0, 225.0, 36.0), (97.0, 76.0, 51.0)),
    ((215.0, 0.0, 99.0), (332.0, 100.0, 42.0)),
    ((10.0, 10.0, 10.0), (0.0, 0.0, 4.0)),
  ];

  asserts.iter().for_each(|tuples| {
    assert!(a(tuples.0, tuples.1));
  });
}
