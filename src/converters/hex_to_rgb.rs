#[cfg(not(feature = "std"))] use alloc::string::String;
#[cfg(not(feature = "std"))] use alloc::vec::Vec;
use crate::{err, ColorTuple};
use err::{make_parse_err, ParseError};


pub fn hex_to_rgb(s: &str) -> Result<ColorTuple, ParseError> {
  let mut hex = s.replace("#", "").to_lowercase();
  let hex_chars = hex.chars().collect::<Vec<char>>();
  let count = hex_chars.len();

  if count == 3 {
    hex = hex_chars
      .iter()
      .map(|c| format!("{}", &c).repeat(2))
      .collect::<Vec<String>>()
      .join("");
  } else if count != 6 {
    return Err(make_parse_err(s, "hex"));
  }

  match usize::from_str_radix(&hex, 16) {
    Ok(num) => Ok(hex_num_to_rgb(num)),
    Err(_) => Err(make_parse_err(s, "hex")),
  }
}

fn hex_num_to_rgb(num: usize) -> ColorTuple {
  let r = (num >> 16) as f64;
  let g = ((num >> 8) & 0x00FF) as f64;
  let b = (num & 0x0000_00FF) as f64;

  (r, g, b)
}
