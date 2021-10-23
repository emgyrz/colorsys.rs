#[cfg(not(feature = "std"))] use alloc::string::String;
use crate::{normalize::round_ratio, ColorTupleA};
use crate::common::{f64_abs, f64_round};

pub fn tuple_to_string(tuple: &ColorTupleA, prefix: &str) -> String {
  let (x, y, z, a) = tuple;
  let mut start = String::from(prefix);
  let a = if f64_abs(a - 1.0) < core::f64::EPSILON {
    String::from("1")
  } else {
    format!("{}", round_ratio(*a))
  };

  let is_hsl = prefix == "hsl";
  let mut result = String::new();
  [x, y, z]
    .iter()
    .enumerate()
    .for_each(|(ind, u)| {
      result.push_str(&format!("{}",  f64_round(**u)));
      if is_hsl && (ind == 1 || ind == 2) {
        result.push('%');
      }
      if ind != 2 {
        result.push(',');
      }
    });

  if a != "1" {
    start.push('a');
    result.push(',');
    result.push_str(&a);
  }

  format!("{}({})", start, result)
}

