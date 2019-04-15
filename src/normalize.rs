use crate::common::approx::approx_def;

use super::consts::{ALL_MIN, HUE_MAX, PERCENT_MAX, RATIO_MAX, RGB_UNIT_MAX};

fn normalize(val: f64, max: f64) -> f64 {
  if val < ALL_MIN {
    return ALL_MIN;
  }
  if val > max {
    return max;
  }
  val
}

pub fn normalize_percent(val: f64) -> f64 {
  normalize(val, PERCENT_MAX)
}

pub fn normalize_hue(h: f64) -> f64 {
  let h = normalize(h, HUE_MAX);
  if (h - HUE_MAX).abs() < std::f64::EPSILON {
    0.0
  } else {
    h
  }
}

pub fn normalize_rgb_unit(val: f64) -> f64 {
  normalize(val, RGB_UNIT_MAX)
}

pub fn normalize_ratio(val: f64) -> f64 {
  normalize(val, RATIO_MAX)
}

pub fn normalize_opt_ratio(val: Option<f64>) -> Option<f64> {
  val.map(normalize_ratio).filter(|al| !approx_def(*al, RATIO_MAX))
}

pub fn bound(r: f64, entire: f64) -> f64 {
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

pub fn bound_ratio(r: f64) -> f64 {
  bound(r, RATIO_MAX)
}

pub fn bound_hue(h: f64) -> f64 {
  bound(h, HUE_MAX)
}

pub fn round_ratio(r: f64) -> f64 {
  (r * 100.0).round() / 100.0
}
