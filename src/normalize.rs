use super::consts::{ALL_MIN, HUE_MAX, PERCENT_MAX, RATIO_MAX, RGB_UNIT_MAX};

fn normalize(val: f32, max: f32) -> f32 {
  if val < ALL_MIN {
    return ALL_MIN;
  }
  if val > max {
    return max;
  }
  val
}

pub fn normalize_percent(val: f32) -> f32 {
  normalize(val, PERCENT_MAX)
}

pub fn normalize_hue(h: f32) -> f32 {
  let h = normalize(h, HUE_MAX);
  if (h - HUE_MAX).abs() < std::f32::EPSILON {
    0.0
  } else {
    h
  }
}

pub fn normalize_rgb_unit(val: f32) -> f32 {
  normalize(val, RGB_UNIT_MAX)
}

pub fn normalize_ratio(val: f32) -> f32 {
  normalize(val, RATIO_MAX)
}

pub fn bound(r: f32, entire: f32) -> f32 {
  let mut n = r;
  loop {
    let less = n < ALL_MIN;
    let bigger = n > entire;
    if !less && !bigger {
      break n;
    }
    if less {
      n += entire;
    } else {
      n -= entire;
    }
  }
}

pub fn bound_ratio(r: f32) -> f32 {
  bound(r, RATIO_MAX)
}

pub fn bound_hue(h: f32) -> f32 {
  bound(h, HUE_MAX)
}

pub fn round_ratio(r: f32) -> f32 {
  (r * 100.0).round() / 100.0
}
