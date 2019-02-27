use super::error::{make_parse_err, ParseError};
use super::{Hex, Hsl, Rgb, Rgba};

pub fn hex(s: &str) -> Result<(String, usize), ParseError> {
  let mut hex = s.replace("#", "").to_lowercase();
  let count = hex.chars().count();

  if count == 3 {
    hex = hex
      .chars()
      .map(|c| c.to_string().repeat(2))
      .collect::<Vec<String>>()
      .join("");
  } else if count != 6 {
    return Err(make_parse_err(s));
  }

  match usize::from_str_radix(&hex, 16) {
    Ok(num) => Ok((hex.to_string(), num)),
    Err(_) => Err(make_parse_err(s)),
  }
}
