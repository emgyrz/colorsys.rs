use super::color::{Color, Hex, Hsl, Rgb};
use super::from_str;
#[cfg(test)]
// use super::converters::{hex_to_rgb, rgb_to_hsl};

// #[test]
// fn hex_to_rgb_test() {
//   assert_eq!(
//     Hex::from_str("#fff").unwrap().to_rgb(),
//     Rgb {
//       r: 255,
//       g: 255,
//       b: 255
//     }
//   );
//   assert_eq!(hex_to_rgb("000").unwrap(), Rgb { r: 0, g: 0, b: 0 });
//   assert_eq!(
//     hex_to_rgb("b7c03e").unwrap(),
//     Rgb {
//       r: 183,
//       g: 192,
//       b: 62
//     }
//   );

//   assert!(hex_to_rgb("123b7c03e").is_err());
// }
#[test]
fn hex_from_str_test() {
  assert_eq!(from_str::hex("#ffcc00").unwrap().0, "ffcc00");
  assert_eq!(from_str::hex("#FA1CBE").unwrap().0, "fa1cbe");
  assert_eq!(from_str::hex("#fc0").unwrap().0, "ffcc00");
  assert_eq!(from_str::hex("fc0").unwrap().0, "ffcc00");
  assert!(from_str::hex("").is_err());
  assert!(from_str::hex("ffcc0g").is_err());
  assert!(from_str::hex("zxc.@0").is_err());
  assert!(from_str::hex("鐵").is_err());
  assert!(from_str::hex("中鏽鏽").is_err());
}

#[test]
fn rgb_to_hsl_test() {
  let asserts = [
    ((255, 255, 255), (0, 0.0, 1.0)),
    ((0, 0, 0), (0, 0.0, 0.0)),
    ((215, 231, 236), (194, 0.355_932_12, 0.884_313_7)),
    ((108, 225, 36), (97, 0.759_036_2, 0.511_764_7)),
    ((215, 0, 99), (332, 1.0, 0.421_568_63)),
    ((10, 10, 10), (0, 0.0, 0.0392_156_88)),
  ];

  asserts.iter().for_each(|a| {
    assert_eq!(Rgb::from_tuple(a.0).to_hsl().as_tuple(), a.1);
  });
}
