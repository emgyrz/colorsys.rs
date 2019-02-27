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
  ParseError {
    message: msg.to_string(),
  }
}

// pub fn make_hex_parse_err(hex: &str) -> ParseError {
//   make_parse_err(&format!("cannot parse string {} as hex", hex))
// }

// impl std::error::Error for ParseError {
//   fn fmt(&self, f: &mut Formatter) -> Result<(),fmt::Error> {
//     write!(f, "{}", self.message)
//   }
// }
