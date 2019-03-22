use super::consts::{HUE_MAX, PERCENT_MAX, RGB_UNIT_MAX};
use super::normalize::{
  bound_hue, bound_ratio, normalize_hue, normalize_percent, normalize_ratio, normalize_rgb_unit,
};
use super::{ColorTuple, ColorTupleA};

fn rgb_to(n: f32) -> f32 {
  normalize_ratio(n / RGB_UNIT_MAX)
}
fn to_rgb(n: f32) -> f32 {
  normalize_rgb_unit(n * RGB_UNIT_MAX)
}
fn hue_to(n: f32) -> f32 {
  bound_hue(n) / HUE_MAX
}
fn to_hue(n: f32) -> f32 {
  bound_ratio(n) * HUE_MAX
}
fn per_to(n: f32) -> f32 {
  normalize_ratio(n / PERCENT_MAX)
}
fn to_per(n: f32) -> f32 {
  normalize_percent(n * PERCENT_MAX)
}

pub fn rgb_to_ratio(t: &ColorTuple) -> ColorTuple {
  (rgb_to(t.0), rgb_to(t.1), rgb_to(t.2))
}

pub fn rgba_to_ratio(t: &ColorTupleA) -> ColorTupleA {
  (rgb_to(t.0), rgb_to(t.1), rgb_to(t.2), normalize_ratio(t.3))
}
pub fn ratio_to_rgb(t: &ColorTuple) -> ColorTuple {
  (to_rgb(t.0), to_rgb(t.1), to_rgb(t.2))
}

pub fn ratio_to_rgba(t: &ColorTupleA) -> ColorTupleA {
  (to_rgb(t.0), to_rgb(t.1), to_rgb(t.2), normalize_ratio(t.3))
}

pub fn hsl_to_ratio(t: &ColorTuple) -> ColorTuple {
  (hue_to(t.0), per_to(t.1), per_to(t.2))
}

pub fn hsla_to_ratio(t: &ColorTupleA) -> ColorTupleA {
  (hue_to(t.0), per_to(t.1), per_to(t.2), normalize_ratio(t.3))
}
pub fn ratio_to_hsl(t: &ColorTuple) -> ColorTuple {
  (to_hue(t.0), to_per(t.1), to_per(t.2))
}

pub fn ratio_to_hsla(t: &ColorTupleA) -> ColorTupleA {
  (to_hue(t.0), to_per(t.1), to_per(t.2), normalize_ratio(t.3))
}
