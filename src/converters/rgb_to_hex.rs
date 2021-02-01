
#[cfg(not(feature = "std"))] use alloc::string::String;

use crate::ColorTuple;
use crate::common::f64_round;

fn to_hex(n: f64) -> String {
  let s = format!("{:x}", f64_round(n) as u32);
  if s.len() == 1 {
    String::from("0") + &s
  } else {
    s
  }
}

pub fn rgb_to_hex(t: &ColorTuple) -> String {
  let (r, g, b) = *t;

  format!("#{}{}{}", to_hex(r), to_hex(g), to_hex(b))
}
