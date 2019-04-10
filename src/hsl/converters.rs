use crate::consts::RGB_UNIT_MAX;
use crate::normalize::bound_ratio;
use crate::ratio_converters::hsl_to_ratio;
use crate::ColorTuple;

static ONE: f64 = 1.0;
static TWO: f64 = 2.0;
static SIX: f64 = 6.0;

static ONE_THIRD: f64 = ONE / 3.0;
static TWO_THIRD: f64 = TWO / 3.0;

pub fn hsl_to_rgb(hsl: &ColorTuple) -> ColorTuple {
  let calc_rgb_unit = |unit: f64, temp1: f64, temp2: f64| -> f64 {
    let mut result = temp2;
    if SIX * unit < ONE {
      result = temp2 + (temp1 - temp2) * SIX * unit
    } else if TWO * unit < ONE {
      result = temp1
    } else if 3.0 * unit < TWO {
      result = temp2 + (temp1 - temp2) * (TWO_THIRD - unit) * SIX
    }
    result * RGB_UNIT_MAX
  };

  let (h, s, l) = hsl_to_ratio(hsl);
  if s == 0.0 {
    let unit = RGB_UNIT_MAX * l;
    return (unit, unit, unit);
  }

  let temp1 = if l < 0.5 { l * (ONE + s) } else { l + s - l * s };

  let temp2 = TWO * l - temp1;
  let hue = h;

  let temp_r = bound_ratio(hue + ONE_THIRD);
  let temp_g = bound_ratio(hue);
  let temp_b = bound_ratio(hue - ONE_THIRD);

  let r = calc_rgb_unit(temp_r, temp1, temp2);
  let g = calc_rgb_unit(temp_g, temp1, temp2);
  let b = calc_rgb_unit(temp_b, temp1, temp2);
  (r, g, b)
}
