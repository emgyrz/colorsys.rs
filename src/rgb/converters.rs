use crate::consts::{ALL_MIN, PERCENT_MAX, RGB_UNIT_MAX};
use crate::ColorTuple;

fn get_min(rgb: &[f32]) -> f32 {
  rgb.iter().fold(std::f32::MAX, |a, &b| a.min(b))
}

fn get_max(rgb: &[f32]) -> f32 {
  rgb.iter().fold(std::f32::MIN, |a, &b| a.max(b))
}

pub fn rgb_to_hsl(rgb: &ColorTuple) -> ColorTuple {
  let (r, g, b) = *rgb;
  let rgb_arr: Vec<f32> = [r, g, b].iter().map(|p| p / RGB_UNIT_MAX).collect();
  let max = get_max(&rgb_arr);
  let min = get_min(&rgb_arr);
  let luminace = (max + min) / 2.0;

  if max.eq(&min) {
    return (ALL_MIN, ALL_MIN, luminace * PERCENT_MAX);
  }

  let max_min_delta = max - min;
  let saturation =
    if luminace > 0.5 { max_min_delta / (2.0 - max - min) } else { max_min_delta / (max + min) };

  let red = rgb_arr[0];
  let green = rgb_arr[1];
  let blue = rgb_arr[2];

  let hue = if red.eq(&max) {
    let x = if g < b { 6.0 } else { ALL_MIN };
    (green - blue) / max_min_delta + x
  } else if green.eq(&max) {
    (blue - red) / max_min_delta + 2.0
  } else {
    (red - green) / max_min_delta + 4.0
  };

  (hue * 60.0, saturation * PERCENT_MAX, luminace * PERCENT_MAX)
}
