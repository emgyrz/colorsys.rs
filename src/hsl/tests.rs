use crate::{ApproxEq, ColorTuple, Hsl, Rgb};

fn round(n: f64) -> u32 {
  n.round() as u32
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
  // let x: ColorTuple = hsl.into();
  // let y = Hsl::from(&x);
  // println!("{:?}", x);
  // println!("{:?}", y);
  // let a = [1.0_f64, 1.0, 12.0, 2.0];
  // let hsl = Hsl::from(&a[..]);
  // let v = vec![1, 2, 3];
  // for a in &v {}

  // let rgb = Rgb::from_hex_str("ffcc00").unwrap();
  // for u in rgb {
  //   println!("{:?}", u);
  // }
}
