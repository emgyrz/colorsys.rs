use crate::{ApproxEq, ColorTuple, Hsl, Rgb};
use crate::common::f64_round;

fn round(n: f64) -> u32 {
  f64_round(n) as u32
}

fn round_tuple(t: &ColorTuple) -> (u32, u32, u32) {
  let (x, y, z) = *t;
  (round(x), round(y), round(z))
}

#[test]
fn hsl_to_rgb() {
  let hsl = Hsl::from((126.0, 43.0, 52.0));
  let rgb = Rgb::from(&hsl);
  assert_eq!(round_tuple(&rgb.as_ref().into()), (80, 185, 90));

  let hsl_new = Hsl::from(&rgb);
  assert!(hsl_new.approx_eq(&hsl));
}
