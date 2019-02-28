use super::normalize::bound_ratio;
use super::{ColorTuple, HexTuple};

fn get_min(rgb: &[f32]) -> f32 {
  rgb.iter().fold(std::f32::MAX, |a, &b| a.min(b))
}

fn get_max(rgb: &[f32]) -> f32 {
  rgb.iter().fold(std::f32::MIN, |a, &b| a.max(b))
}

pub fn rgb_to_hex(rgb: &ColorTuple) -> HexTuple {
  fn to_hex(n: f32) -> String {
    let s = format!("{:x}", n.round() as i32);
    if s.len() == 1 {
      "0".to_string() + &s
    } else {
      s
    }
  }
  let (r, g, b) = *rgb;

  (to_hex(r), to_hex(g), to_hex(b))
}

pub fn rgb_to_hsl(rgb: &ColorTuple) -> ColorTuple {
  let (r, g, b) = *rgb;
  let rgb_arr: Vec<f32> = [r, g, b].iter().map(|p| p / 255.0).collect();
  let max = get_max(&rgb_arr);
  let min = get_min(&rgb_arr);
  let luminace = (max + min) / 2.0;

  if max.eq(&min) {
    return (0.0, 0.0, luminace);
  }

  let max_min_delta = max - min;
  let saturation =
    if luminace > 0.5 { max_min_delta / (2.0 - max - min) } else { max_min_delta / (max + min) };

  let red = rgb_arr[0];
  let green = rgb_arr[1];
  let blue = rgb_arr[2];

  let hue = if red.eq(&max) {
    let x = if g < b { 6.0 } else { 0.0 };
    (green - blue) / max_min_delta + x
  } else if green.eq(&max) {
    (blue - red) / max_min_delta + 2.0
  } else {
    (red - green) / max_min_delta + 4.0
  };

  let hue = hue * 60.0;

  (hue, saturation, luminace)
}

fn calc_rgb_unit(unit: f32, temp1: f32, temp2: f32) -> f32 {
  let mut result = temp2;
  if 6.0 * unit < 1.0 {
    result = temp2 + (temp1 - temp2) * 6.0 * unit
  } else if 2.0 * unit < 1.0 {
    result = temp1
  } else if 3.0 * unit < 2.0 {
    result = temp2 + (temp1 - temp2) * (2.0 / 3.0 - unit) * 6.0
  }
  result * 255.0
}
pub fn hsl_to_rgb(hsl: &ColorTuple) -> ColorTuple {
  let (h, s, l) = *hsl;
  if s == 0.0 {
    let unit = 255.0 * l;
    return (unit, unit, unit);
  }

  let temp1 = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };

  let temp2 = 2.0 * l - temp1;
  let hue = h / 360.0;

  let one_third = 1.0 / 3.0;
  let temp_r = bound_ratio(hue + one_third);
  let temp_g = bound_ratio(hue);
  let temp_b = bound_ratio(hue - one_third);

  let r = calc_rgb_unit(temp_r, temp1, temp2);
  let g = calc_rgb_unit(temp_g, temp1, temp2);
  let b = calc_rgb_unit(temp_b, temp1, temp2);
  (r, g, b)
}

pub fn hex_num_to_rgb(num: usize) -> ColorTuple {
  let r = (num >> 16) as f32;
  let g = ((num >> 8) & 0x00FF) as f32;
  let b = (num & 0x0000_00FF) as f32;

  (r, g, b)
}

pub fn as_rounded_rgb_tuple(t: &ColorTuple) -> (u16, u16, u16) {
  let (r, g, b) = *t;
  (r.round() as u16, g.round() as u16, b.round() as u16)
}

pub fn ratio_as_percent(r: f32) -> u16 {
  (r * 100.0).round() as u16
}

pub fn as_rounded_hsl_tuple(t: &ColorTuple) -> (u16, u16, u16) {
  let (h, s, l) = *t;
  (h.round() as u16, ratio_as_percent(s), ratio_as_percent(l))
}

pub fn round_ratio(r: f32) -> f32 {
  (r * 100.0).round() / 100.0
}
