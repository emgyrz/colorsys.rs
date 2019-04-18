use crate::{normalize::round_ratio, ColorTupleA};

pub fn tuple_to_string(tuple: &ColorTupleA, prefix: &str) -> String {
  let (x, y, z, a) = tuple;
  let mut start = String::from(prefix);
  let a = if (a - 1.0).abs() < std::f64::EPSILON {
    String::from("1")
  } else {
    round_ratio(*a).to_string()
  };

  let mut vals =
    [x, y, z].iter().map(|u| (u.round() as u8).to_string()).collect::<Vec<String>>().join(",");

  if a != "1" {
    start.push('a');
    vals.push_str(&(",".to_owned() + &a));
  }

  format!("{}({})", start, vals)
}
