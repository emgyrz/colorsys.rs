use crate::{ColorTuple, Rgb};

fn round(n: f32) -> u32 {
  n.round() as u32
}

fn round_tuple(t: &ColorTuple) -> (u32, u32, u32) {
  let (x, y, z) = *t;
  (round(x), round(y), round(z))
}

#[test]
fn lighten() {
  let mut rgb = Rgb::from(80.0, 186.0, 90.0);
  let mut rgb2 = rgb.clone();
  let mut rgb3 = rgb.clone();
  let mut rgb4 = rgb.clone();

  rgb.lighten(15.0);
  assert_eq!(round_tuple(&rgb.as_tuple()), (135, 208, 142));

  rgb2.lighten(45.0);
  assert_eq!(round_tuple(&rgb2.as_tuple()), (245, 251, 245));

  rgb3.lighten(-23.0);
  assert_eq!(round_tuple(&rgb3.as_tuple()), (42, 107, 48));

  rgb4.lighten(-203.0);
  assert_eq!(round_tuple(&rgb4.as_tuple()), (0, 0, 0));
}
