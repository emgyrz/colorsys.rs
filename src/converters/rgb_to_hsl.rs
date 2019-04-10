use crate::consts::{ALL_MIN, PERCENT_MAX};
use crate::ratio_converters::rgb_to_ratio;
use crate::ColorTuple;

enum RgbUnit {
  Red,
  Green,
  Blue,
}

fn get_max(red: f64, green: f64, blue: f64) -> (f64, RgbUnit) {
  if (red > green) && (red > blue) {
    return (red, RgbUnit::Red);
  }
  if green > blue {
    (green, RgbUnit::Green)
  } else {
    (blue, RgbUnit::Blue)
  }
}

fn get_min(red: f64, green: f64, blue: f64) -> f64 {
  if (red < green) && (red < blue) {
    red
  } else if green < blue {
    green
  } else {
    blue
  }
}

pub fn rgb_to_hsl(rgb: &ColorTuple) -> ColorTuple {
  let (red, green, blue) = rgb_to_ratio(&rgb);
  let (max, max_unit) = get_max(red, green, blue);
  let min = get_min(red, green, blue);
  let max_plus_min = max + min;
  let luminace = (max_plus_min) / 2.0;

  if max.eq(&min) {
    return (ALL_MIN, ALL_MIN, luminace * PERCENT_MAX);
  }

  let max_min_delta = max - min;
  let saturation = if luminace > 0.5 {
    max_min_delta / (2.0 - max_min_delta)
  } else {
    max_min_delta / (max_plus_min)
  };

  let hue = match max_unit {
    RgbUnit::Red => {
      let x = if green < blue { 6.0 } else { ALL_MIN };
      (green - blue) / max_min_delta + x
    }
    RgbUnit::Green => (blue - red) / max_min_delta + 2.0,
    RgbUnit::Blue => (red - green) / max_min_delta + 4.0,
  };

  (hue * 60.0, saturation * PERCENT_MAX, luminace * PERCENT_MAX)
}
