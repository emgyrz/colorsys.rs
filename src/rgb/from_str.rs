use super::converters::hex_num_to_rgb;
use crate::err::{make_parse_err, ParseError};
use crate::{consts, ColorTuple, ColorTupleA};

use consts::{ALL_MIN, RGB_UNIT_MAX};

// use super::converters::hex_num_to_rgb;

pub fn hex(s: &str) -> Result<ColorTuple, ParseError> {
  let mut hex = s.replace("#", "").to_lowercase();
  let hex_chars = hex.chars().collect::<Vec<char>>();
  let count = hex_chars.len();

  if count == 3 {
    hex = hex_chars.iter().map(|c| c.to_string().repeat(2)).collect::<Vec<String>>().join("");
  } else if count != 6 {
    return Err(make_parse_err(s, "hex"));
  }

  match usize::from_str_radix(&hex, 16) {
    Ok(num) => Ok(hex_num_to_rgb(num)),
    Err(_) => Err(make_parse_err(s, "hex")),
  }
}

pub fn rgb(s: &str) -> Result<ColorTupleA, ParseError> {
  let make_err = || Err(make_parse_err(s, "rgb or rgba"));
  let s = s.trim().to_lowercase().replace(" ", "");
  let is_rgb = s.starts_with("rgb(");
  let is_rgba = s.starts_with("rgba(");
  let is_ends_with_braket = s.ends_with(')');

  if (!is_rgb && !is_rgba) || !is_ends_with_braket {
    return make_err();
  }
  let start_ind = if is_rgb { 4 } else { 5 };
  let s = &s[start_ind..s.len() - 1];
  let nums_str = s.split(',').collect::<Vec<&str>>();
  let len = nums_str.len();
  if is_rgb && len != 3 || is_rgba && len != 4 {
    return make_err();
  }

  let mut nums = Vec::with_capacity(len);
  for n in nums_str {
    match n.parse::<f32>() {
      Ok(num) => {
        if num < ALL_MIN || num > RGB_UNIT_MAX {
          return make_err();
        }
        nums.push(num)
      }
      Err(_) => {
        return make_err();
      }
    }
  }

  let alpha = if is_rgba { nums[len - 1] } else { 1.0 };

  Ok((nums[0], nums[1], nums[2], alpha))
}
