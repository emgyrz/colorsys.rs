use std::fmt;

#[derive(Clone)]
pub struct ParseError {
  pub message: String,
}

impl fmt::Debug for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "{}", self.message)
  }
}

pub fn make_parse_err(msg: &str) -> ParseError {
  ParseError { message: msg.to_string() }
}

pub fn make_def_parse_err(str_in: &str) -> ParseError {
  make_parse_err(&format!("cannot parse string `{}` as color", str_in))
}

pub fn make_hex_parse_err(str_in: &str) -> ParseError {
  make_parse_err(&format!("cannot parse string `{}` as hex", str_in))
}

pub fn make_rgb_parse_err(str_in: &str) -> ParseError {
  make_parse_err(&format!("cannot parse string `{}` as rgb", str_in))
}

pub fn make_rgba_parse_err(str_in: &str) -> ParseError {
  make_parse_err(&format!("cannot parse string `{}` as rgba", str_in))
}

pub fn make_hsl_parse_err(str_in: &str) -> ParseError {
  make_parse_err(&format!("cannot parse string `{}` as hsl", str_in))
}

pub fn make_hsla_parse_err(str_in: &str) -> ParseError {
  make_parse_err(&format!("cannot parse string `{}` as hsla", str_in))
}

// impl std::error::Error for ParseError {
//   fn fmt(&self, f: &mut Formatter) -> Result<(),fmt::Error> {
//     write!(f, "{:?}", self.message)
//   }
// }
