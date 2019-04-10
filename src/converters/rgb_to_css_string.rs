use crate::normalize::round_ratio;
use crate::Rgb;

pub fn rgb_to_css_string(rgb: &Rgb) -> String {
  let (r, g, b) = rgb.into();
  let a = rgb.get_alpha();
  let alpha =
    if (a - 1.0) < std::f64::EPSILON { "1".to_owned() } else { round_ratio(a).to_string() };
  let mut start = String::from("rgb");
  let mut vals =
    [r, g, b].iter().map(|u| (u.round() as u8).to_string()).collect::<Vec<String>>().join(",");

  if alpha != "1" {
    start.push('a');
    vals.push_str(&(",".to_owned() + &alpha));
  }

  format!("{}({})", start, vals)
}
