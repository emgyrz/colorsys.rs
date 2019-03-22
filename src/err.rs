use std::fmt;

#[derive(Clone)]
pub struct ParseError {
  pub message: String,
}

fn fmt(f: &mut fmt::Formatter, msg: &str) -> Result<(), fmt::Error> {
  write!(f, "{}", msg)
}

impl fmt::Debug for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    fmt(f, &self.message)
  }
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    fmt(f, &self.message)
  }
}

pub fn make_parse_err(s: &str, col_type: &str) -> ParseError {
  ParseError { message: format!("cannot parse string `{}` as {}", s, col_type) }
}

impl std::error::Error for ParseError {}
