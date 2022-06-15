#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use consts::{ALL_MIN, RATIO_MAX, RGB_UNIT_MAX};

use crate::{ColorTuple, consts};
use crate::err::{make_parse_err, ParseError};

pub fn rgb(s: &str) -> Result<(ColorTuple, Option<f64>), ParseError> {
  let make_err = || Err(make_parse_err(s, "rgb or rgba"));
  let s = s.trim().to_lowercase().replace(' ', "");
  let is_rgb = s.starts_with("rgb(");
  let is_rgba = s.starts_with("rgba(");
  let is_ends_with_bracket = s.ends_with(')');

  if (!is_rgb && !is_rgba) || !is_ends_with_bracket {
    return make_err();
  }
  let start_ind = if is_rgb { 4 } else { 5 };
  let s = &s[start_ind..s.len() - 1];
  let nums_str = s.split(',').collect::<Vec<&str>>();
  let len = nums_str.len();
  if (is_rgb && len != 3) || (is_rgba && len != 4) {
    return make_err();
  }

  let mut nums = Vec::with_capacity(len);
  for (ind, n) in nums_str.iter().enumerate() {
    if let Ok(num) = n.parse::<f64>() {
      let max = if ind == 4 { RATIO_MAX } else { RGB_UNIT_MAX };
      if num < ALL_MIN || num > max {
        return make_err();
      }
      nums.push(num)
    } else {
      return make_err();
    }
  }
  let rgb = (nums[0], nums[1], nums[2]);
  let alpha = if is_rgba { Some(nums[len - 1]) } else { None };

  Ok((rgb, alpha))
}
