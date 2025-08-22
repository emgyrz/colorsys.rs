#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use super::Hs;
use crate::err::{ParseError, make_parse_err};
use crate::{ColorTuple, consts};

use consts::{ALL_MIN, HUE_MAX, PERCENT_MAX, RATIO_MAX};

fn get_max_by_ind(ind: usize) -> f64 {
  match ind {
    0 => HUE_MAX,
    3 => RATIO_MAX,
    _ => PERCENT_MAX,
  }
}

fn strings_from_name(space: Hs) -> (String, String, String) {
  let mut x = String::with_capacity(1);
  match space {
    Hs::Hsl => {
      x.push('l');
    }
    Hs::Hsv => {
      x.push('v');
    }
  }

  (format!("hs{}(", x), format!("hs{}a(", x), format!("hs{} or hs{}a", x, x))
}

pub fn hsl_hsv_from_str(
  s: &str,
  col_space: Hs,
) -> Result<(ColorTuple, Option<f64>), ParseError> {
  let (start, start_a, err_name) = strings_from_name(col_space);

  let make_err = || Err(make_parse_err(s, &err_name));
  let s = s.trim().to_lowercase().replace([' ', '%'], "");
  let is_hsl = s.starts_with(&start);
  let is_hsla = s.starts_with(&start_a);
  let is_ends_with_bracket = s.ends_with(')');

  if (!is_hsl && !is_hsla) || !is_ends_with_bracket {
    return make_err();
  }
  let start_ind = if is_hsl { 4 } else { 5 };
  let s = &s[start_ind..s.len() - 1];
  let nums_str = s.split(',').collect::<Vec<&str>>();
  let len = nums_str.len();
  if (is_hsl && len != 3) || (is_hsla && len != 4) {
    return make_err();
  }

  let mut nums = Vec::with_capacity(len);
  for (ind, n) in nums_str.iter().enumerate() {
    if let Ok(num) = n.parse::<f64>() {
      let max = get_max_by_ind(ind);
      if num < ALL_MIN || num > max {
        return make_err();
      }
      nums.push(num)
    } else {
      return make_err();
    }
  }
  let hsl = (nums[0], nums[1], nums[2]);
  let alpha = if is_hsla { Some(nums[len - 1]) } else { None };

  Ok((hsl, alpha))
}
