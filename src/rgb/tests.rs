use crate::{ColorTransform, ColorTuple, ColorTupleA, ParseError, Rgb};
use crate::common::f64_round;

fn round(n: f64) -> u32 {
  f64_round(n) as u32
}

fn round_tuple(t: &ColorTuple) -> (u32, u32, u32) {
  let (x, y, z) = *t;
  (round(x), round(y), round(z))
}

#[test]
fn lighten() {
  let mut rgb = Rgb::from((80.0, 186.0, 90.0));
  let mut rgb2 = rgb.clone();
  let mut rgb3 = rgb.clone();
  let mut rgb4 = rgb.clone();

  rgb.lighten(15.0);

  assert_eq!(round_tuple(&rgb.into()), (135, 208, 142));

  rgb2.lighten(45.0);
  assert_eq!(round_tuple(&rgb2.into()), (245, 251, 245));

  rgb3.lighten(-23.0);
  assert_eq!(round_tuple(&rgb3.into()), (42, 107, 48));

  rgb4.lighten(-203.0);
  assert_eq!(round_tuple(&rgb4.into()), (0, 0, 0));
}

#[test]
fn from_str_tst() {
  fn parse_rgb(s: &str) -> Result<Rgb, ParseError> {
    s.parse::<Rgb>()
  }

  assert_eq!(
    Into::<ColorTuple>::into(parse_rgb("Rgb(134,11,251)").unwrap()),
    Rgb::from((134.0, 11.0, 251.0)).into()
  );

  assert_eq!(
    Into::<ColorTupleA>::into(
      parse_rgb("rgba(134.9,11.1,250.55, 0.9)").unwrap()
    ),
    Rgb::from((134.9, 11.1, 250.55, 0.9)).into()
  );
  assert_eq!(
    Into::<ColorTuple>::into(parse_rgb("rgb (0,   0,0)").unwrap()),
    Rgb::default().into()
  );

  assert!(parse_rgb("12,1,97)").is_err());
  assert!(parse_rgb("").is_err());
  assert!(parse_rgb("ffcc0g").is_err());
}

#[test]
fn to_hex_tst() {
  let rgba = Rgb::new(57.3, 12.7, 53.0, Some(0.33));
  assert_eq!("#390d35", rgba.to_hex_string());
  assert_eq!("#390d3554", rgba.to_hexa_string());

  let rgba2 = Rgb::new(67.0, 253.0, 101.0, Some(0.85));
  assert_eq!("#43fd65", rgba2.to_hex_string());
  assert_eq!("#43fd65d9", rgba2.to_hexa_string());
}

#[test]
fn rgb_iter() {
  let rgb1 = Rgb::from_hex_str("37ea4c").unwrap();
  let rgb2 = Rgb::from_hex_str("ffcc00").unwrap();
  let _t: ColorTuple = rgb1.as_ref().into();
  let _rgb3 = &rgb1 + &rgb2;
  // println!(">>> {:?}", rgb3);
  // println!(">>> {:?}", t);
}

#[test]
#[rustfmt::skip]
fn rgb_from() {
  let rgb1 = Rgb::from_hex_str("37ea4c").unwrap();

  let rgb2 = Rgb::from(
    Into::<[f32; 4]>::into(Rgb::from(
      Into::<[u8; 3]>::into(
        Rgb::from(
          Into::<(i32, i32, i32)>::into(
            Rgb::from(
              Into::<[i64; 3]>::into(&rgb1)
            )
          )
        )
      )
    ))
  );

  assert_eq!(rgb1, rgb2);
}

