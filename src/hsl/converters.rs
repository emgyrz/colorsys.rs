use crate::consts::RGB_UNIT_MAX;
use crate::normalize::bound_ratio;
use crate::ratio_converters::hsl_to_ratio;
use crate::ColorTuple;

fn calc_rgb_unit(unit: f32, temp1: f32, temp2: f32) -> f32 {
  let mut result = temp2;
  if 6.0 * unit < 1.0 {
    result = temp2 + (temp1 - temp2) * 6.0 * unit
  } else if 2.0 * unit < 1.0 {
    result = temp1
  } else if 3.0 * unit < 2.0 {
    result = temp2 + (temp1 - temp2) * (2.0 / 3.0 - unit) * 6.0
  }
  result * RGB_UNIT_MAX
}

pub fn hsl_to_rgb(hsl: &ColorTuple) -> ColorTuple {
  let (h, s, l) = hsl_to_ratio(hsl);
  if s == 0.0 {
    let unit = RGB_UNIT_MAX * l;
    return (unit, unit, unit);
  }

  let temp1 = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };

  let temp2 = 2.0 * l - temp1;
  let hue = h;

  let one_third = 1.0 / 3.0;
  let temp_r = bound_ratio(hue + one_third);
  let temp_g = bound_ratio(hue);
  let temp_b = bound_ratio(hue - one_third);

  let r = calc_rgb_unit(temp_r, temp1, temp2);
  let g = calc_rgb_unit(temp_g, temp1, temp2);
  let b = calc_rgb_unit(temp_b, temp1, temp2);
  (r, g, b)
}
