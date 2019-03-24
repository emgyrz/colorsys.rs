use crate::{ColorTuple, ParseError, Rgb};

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

#[test]
fn from_str_tst() {
  fn parse_rgb(s: &str) -> Result<Rgb, ParseError> {
    s.parse::<Rgb>()
  }

  assert_eq!(
    parse_rgb("Rgb(134,11,251)").unwrap().as_tuple(),
    Rgb::from(134.0, 11.0, 251.0).as_tuple()
  );
  assert_eq!(
    parse_rgb("rgba(134.9,11.1,250.55, 0.9)").unwrap().as_tuple_with_alpha(),
    Rgb::from_with_alpha(134.9, 11.1, 250.55, 0.9).as_tuple_with_alpha()
  );
  assert_eq!(parse_rgb("rgb (0,   0,0)").unwrap().as_tuple(), Rgb::default().as_tuple());

  assert!(parse_rgb("12,1,97)").is_err());
  assert!(parse_rgb("").is_err());
  assert!(parse_rgb("ffcc0g").is_err());
}

#[test]
fn tst() {
  // let x: f32 = 3.92354325154235423542435;

  // println!("{}", x.round() as u16);
}
