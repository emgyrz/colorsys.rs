use super::converters::hex_num_to_rgb;
use super::error::{
  make_def_parse_err, make_hex_parse_err, make_hsl_parse_err, make_hsla_parse_err,
  make_rgb_parse_err, make_rgba_parse_err, ParseError,
};
use super::{ColorTuple, ColorTupleA};

pub fn hex(s: &str) -> Result<ColorTuple, ParseError> {
  let mut hex = s.replace("#", "").to_lowercase();
  let count = hex.chars().count();

  if count == 3 {
    hex = hex.chars().map(|c| c.to_string().repeat(2)).collect::<Vec<String>>().join("");
  } else if count != 6 {
    return Err(make_hex_parse_err(s));
  }

  match usize::from_str_radix(&hex, 16) {
    Ok(num) => Ok(hex_num_to_rgb(num)),
    Err(_) => Err(make_hex_parse_err(s)),
  }
}

fn clear_str(s: &str) -> String {
  let mut result = s.to_lowercase();
  for p in ["rgba", "rgb", "hsla", "hsl", "(", ")", "%", " "].iter() {
    result = result.replace(p, "");
  }
  result
}

fn collect_vec_and_parse(s: &str) -> Result<Vec<f32>, std::num::ParseFloatError> {
  let v = s.split(',').map(|c| c.to_string()).collect::<Vec<String>>();

  let mut units = Vec::new();

  for unit in v {
    let u = unit.parse::<f32>()?;
    units.push(u);
  }

  Ok(units)
}

fn parse_color_tuple(s: &str) -> Result<ColorTuple, ParseError> {
  match collect_vec_and_parse(&clear_str(s)) {
    Ok(num_vec) => {
      if num_vec.len() != 3 {
        Err(make_def_parse_err(&s))
      } else {
        Ok((num_vec[0], num_vec[1], num_vec[2]))
      }
    }
    Err(_) => Err(make_def_parse_err(&s)),
  }
}

fn parse_color_tuple_a(s: &str) -> Result<ColorTupleA, ParseError> {
  match collect_vec_and_parse(&clear_str(s)) {
    Ok(num_vec) => {
      if num_vec.len() != 4 {
        Err(make_def_parse_err(&s))
      } else {
        Ok((num_vec[0], num_vec[1], num_vec[2], num_vec[3]))
      }
    }
    Err(_) => Err(make_def_parse_err(&s)),
  }
}

pub fn rgb(s: &str) -> Result<ColorTuple, ParseError> {
  match parse_color_tuple(s) {
    Err(_) => Err(make_rgb_parse_err(s)),
    Ok(rgb) => Ok(rgb),
  }
}

pub fn rgba(s: &str) -> Result<ColorTupleA, ParseError> {
  match parse_color_tuple_a(s) {
    Err(_) => Err(make_rgba_parse_err(s)),
    Ok(rgba) => Ok(rgba),
  }
}

pub fn hsl(s: &str) -> Result<ColorTuple, ParseError> {
  match parse_color_tuple(s) {
    Err(_) => Err(make_hsl_parse_err(s)),
    Ok(hsl) => Ok(hsl),
  }
}

pub fn hsla(s: &str) -> Result<ColorTupleA, ParseError> {
  match parse_color_tuple_a(s) {
    Err(_) => Err(make_hsla_parse_err(s)),
    Ok(hsla) => Ok(hsla),
  }
}
