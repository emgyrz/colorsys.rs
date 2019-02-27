use super::color::RgbTuple;
use super::error;
use super::{Hex, Hsl, Rgb, Rgba};

const HUE_MAX: u16 = 360;
const U8_MAX: f32 = 255.0;

// fn bound(n: f32) -> f32 {
//   let mut n = if n.eq(&std::f32::EPSILON) { 255.0 } else { n };
//   n = get_min(&[255.0, get_max(&[0.0, n])]);
//   if (n - 255.0).abs() < 0.000_001 {
//     return 1.0;
//   }
//   n % 255.0 / 255.0
// }

fn get_min(rgb: &[f32]) -> f32 {
  rgb.iter().fold(std::f32::MAX, |a, &b| a.min(b))
}

fn get_max(rgb: &[f32]) -> f32 {
  rgb.iter().fold(std::f32::MIN, |a, &b| a.max(b))
}

fn nomalize_hsl(t: &(f32, f32, f32)) -> Hsl {
  let (h, s, l) = t;
  let h = normalize(NormalizeArg {
    val: h * 60.0,
    min: 0.0,
    max: 360.0,
  });
  let s = normalize_until_one(*s);
  let l = normalize_until_one(*l);
  Hsl::from_tuple((h as u16, s, l))
}
pub fn rgb_to_hsl(rgb: &Rgb) -> Hsl {
  let (r, g, b) = rgb.as_tuple();
  let rgb_arr: Vec<f32> = [r, g, b].iter().map(|p| f32::from(*p) / U8_MAX).collect();
  let max = get_max(&rgb_arr);
  let min = get_min(&rgb_arr);
  let luminace = (max + min) / 2.0;

  if max.eq(&min) {
    return nomalize_hsl(&(0.0, 0.0, luminace));
  }

  let max_min_delta = max - min;
  let saturation = if luminace > 0.5 {
    max_min_delta / (2.0 - max - min)
  } else {
    max_min_delta / (max + min)
  };

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

  nomalize_hsl(&(hue, saturation, luminace))
}

// fn get_min(nums: &[f32]) -> f32 {
//   nums.iter().fold(std::f32::MAX, |a, &b| a.min(b))
// }

// fn get_max(nums: &[f32]) -> f32 {
//   nums.iter().fold(std::f32::MIN, |a, &b| a.max(b))
// }
// pub fn rgb_to_hsl(rgb: [u8; 3]) -> [f32; 3] {
//   let rgb: Vec<f32> = rgb
//     .iter()
//     .map(|col_part| bound(f32::from(*col_part)))
//     .collect();

//   let r = rgb[0];
//   let g = rgb[1];
//   let b = rgb[2];

//   let max = get_max(&rgb);
//   let min = get_min(&rgb);

//   let mut h = (max + min) / 2.0;
//   let mut s = (max + min) / 2.0;
//   let l = (max + min) / 2.0;
//   if max.eq(&min) {
//     h = 0.0;
//     s = 0.0;
//   } else {
//     let d = max - min;
//     s = if l > 0.5 {
//       d / (2.0 - max - min)
//     } else {
//       d / (max + min)
//     };

//     if r.eq(&max) {
//       let x = if g < b { 6.0 } else { 0.0 };
//       h = (g - b) / d + x;
//     } else if g.eq(&max) {
//       h = (b - r) / d + 2.0;
//     } else if b.eq(&max) {
//       h = (r - g) / d + 4.0;
//     }

//     h /= 6.0;
//   }

//   [h, s, l]
// }

pub fn hex_to_rgb(hex: &Hex) -> (u8, u8, u8) {
  let n = hex.get_num();

  let r = (n >> 16) as u8;
  let g = ((n >> 8) & 0x00FF) as u8;
  let b = (n & 0x0000_00FF) as u8;

  (r, g, b)
}

pub fn rgb_to_hex(rgb: &Rgb) -> (String, usize) {
  fn to_hex(n: u8) -> String {
    let s = format!("{:x}", n);
    if s.len() == 1 {
      "0".to_string() + &s
    } else {
      s
    }
  }
  let (r, g, b) = rgb.as_tuple();
  let value = format!("{}{}{}", to_hex(r), to_hex(g), to_hex(b));
  let num = usize::from_str_radix(&value, 16).unwrap();
  (value, num)
}

pub struct NormalizeArg<T> {
  pub val: T,
  pub max: T,
  pub min: T,
}
pub fn normalize<T: std::cmp::PartialOrd>(arg: NormalizeArg<T>) -> T {
  let NormalizeArg { val, max, min } = arg;
  if val < min {
    return min;
  }
  if val > max {
    return max;
  }
  val
}

pub fn normalize_until_one(a: f32) -> f32 {
  normalize(NormalizeArg {
    val: a,
    min: 0.0,
    max: 1.0,
  })
}

pub fn normalize_hue(a: u16) -> u16 {
  normalize(NormalizeArg {
    val: a,
    min: 0,
    max: HUE_MAX,
  })
}
