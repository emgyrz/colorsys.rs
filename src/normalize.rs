use super::consts::{ALL_MIN, ALPHA_MAX, HUE_MAX, PERCENT_MAX, RGB_UNIT_MAX};

fn bound(val: f32, max: f32) -> f32 {
  if val < ALL_MIN {
    return ALL_MIN;
  }
  if val > max {
    return max;
  }
  val
}

pub fn normalize_percent(val: f32) -> f32 {
  bound(val, PERCENT_MAX)
}

pub fn normalize_hue(h: f32) -> f32 {
  let h = bound(h, HUE_MAX);
  if (h - HUE_MAX).abs() < std::f32::EPSILON {
    0.0
  } else {
    h
  }
}

pub fn normalize_rgb_unit(val: f32) -> f32 {
  bound(val, RGB_UNIT_MAX)
}

pub fn normalize_alpha(val: f32) -> f32 {
  bound(val, ALPHA_MAX)
}
