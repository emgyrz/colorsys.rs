use crate::{normalize::round_ratio, ColorTupleA};

pub fn tuple_to_string(tuple: &ColorTupleA, prefix: &str) -> String {
  let (x, y, z, a) = tuple;
  let mut start = String::from(prefix);
  let a = if (a - 1.0).abs() < std::f64::EPSILON {
    String::from("1")
  } else {
    round_ratio(*a).to_string()
  };

  let is_hsl = prefix == "hsl";
  let mut vals = [x, y, z]
    .iter()
    .enumerate()
    .map(|(ind, u)| {
      let mut s = u.round().to_string();
      if is_hsl && (ind == 1 || ind == 2) {
        s.push('%');
      }
      s
    })
    .collect::<Vec<String>>()
    .join(",");

  if a != "1" {
    start.push('a');
    vals.push_str(&(",".to_owned() + &a));
  }

  format!("{}({})", start, vals)
}
