use crate::ColorTuple;

fn to_hex(n: f64) -> String {
  let s = format!("{:x}", n.round() as u32);
  if s.len() == 1 {
    "0".to_string() + &s
  } else {
    s
  }
}

pub fn rgb_to_hex(t: &ColorTuple) -> String {
  let (r, g, b) = *t;

  format!("#{}{}{}", to_hex(r), to_hex(g), to_hex(b))
}
