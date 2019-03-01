use super::converters::{
  as_rounded_hsl_tuple, as_rounded_rgb_tuple, hsl_to_rgb, ratio_as_percent, rgb_to_hex, rgb_to_hsl,
};
use super::{Color, ColorUnit, Hex, Hsl, Hsla, ParseError, Rgb, RgbColor, Rgba};

#[test]
fn hsl_to_rgb_test() {
  let asserts = [
    ((200.0, 1.0, 0.3), (0, 102, 153)),
    ((192.0, 0.67, 0.28), (24, 100, 119)),
    ((48.0, 0.7, 0.5), (217, 181, 38)),
    ((359.0, 0.33, 0.77), (216, 177, 178)),
  ];

  for (hsl, rgb) in asserts.iter() {
    assert_eq!(as_rounded_rgb_tuple(&hsl_to_rgb(hsl)), *rgb);
  }
}

#[test]
fn rgb_to_hsl_test() {
  let asserts = [
    ((255.0, 255.0, 255.0), (0, 0, 100)),
    ((0.0, 0.0, 0.0), (0, 0, 0)),
    ((215.0, 231.0, 236.0), (194, 36, 88)),
    ((108.0, 225.0, 36.0), (97, 76, 51)),
    ((215.0, 0.0, 99.0), (332, 100, 42)),
    ((10.0, 10.0, 10.0), (0, 0, 4)),
  ];

  asserts.iter().for_each(|a| {
    assert_eq!(as_rounded_hsl_tuple(&Rgb::from_tuple(a.0).to_hsl().as_tuple()), a.1);
  });
}

#[test]
fn lighten_darken_test() {
  let asserts = [
    ((30.0, 108.0, 77.0), 0.2, (52, 188, 134)),
    ((30.0, 108.0, 77.0), 0.9, (255, 255, 255)),
    ((30.0, 108.0, 77.0), -0.2, (8, 28, 20)),
    ((0.0, 0.0, 0.0), 0.5, (128, 128, 128)),
    ((0.0, 0.0, 0.0), -0.5, (0, 0, 0)),
    ((0.0, 0.0, 0.0), 30.5, (255, 255, 255)),
  ];

  for a in asserts.iter() {
    let (origin, amt, result) = *a;
    let rgb = Rgb::from_tuple(origin);
    let mutated = rgb.lighten(amt);
    assert_eq!(as_rounded_rgb_tuple(&mutated.as_tuple()), result);
  }
}

#[test]
fn saturate_desaturate_test() {
  let asserts = [
    ((120.0, 30.0, 90.0), 0.2, (135, 15, 95)),
    ((120.0, 30.0, 90.0), -0.2, (105, 45, 85)),
    ((13.0, 55.0, 137.0), 0.3, (0, 51, 150)),
  ];

  for a in asserts.iter() {
    let (origin, amt, result) = *a;
    let rgb = Rgb::from_tuple(origin);
    let mutated = rgb.saturate(amt);
    assert_eq!(as_rounded_rgb_tuple(&mutated.as_tuple()), result);
  }
}

#[test]
fn adjust_hue_test() {
  let asserts = [
    ((13.0, 55.0, 137.0), 3.0, (13, 49, 137)),
    ((136.0, 17.0, 17.0), 45.0, (136, 106, 17)),
    ((130.0, 255.0, 17.0), 25.0, (31, 255, 17)),
    ((24.0, 91.0, 203.0), -170.0, (203, 166, 24)),
  ];

  for a in asserts.iter() {
    let (origin, amt, result) = *a;
    let rgb = Rgb::from_tuple(origin);
    let mutated = rgb.adjust_hue(amt);
    assert_eq!(as_rounded_rgb_tuple(&mutated.as_tuple()), result);
  }
}

#[test]
fn adjust_color_test() {
  let rgb = Rgb::from_tuple((24.0, 91.0, 203.0));
  let rgb = rgb.adjust_color(RgbColor::Red, 55.0);
  assert_eq!(rgb.as_tuple(), (79.0, 91.0, 203.0));
  let asserts = [
    ((24.0, 0.9, 0.2), RgbColor::Red, 63.0, (160, 42, 5)),
    ((324.0, 0.77, 0.52), RgbColor::Green, 122.0, (227, 160, 151)),
    ((195.0, 0.31, 0.87), RgbColor::Blue, -39.0, (212, 227, 193)),
  ];

  for a in asserts.iter() {
    let (origin, col_name, col_val, result) = *a;
    let hsl = Hsl::from_tuple(origin);
    let mutated = hsl.adjust_color(col_name, col_val);
    assert_eq!(as_rounded_rgb_tuple(&mutated.to_rgb().as_tuple()), result);
  }
}

#[test]
fn to_css_test() {
  let rgb = Rgb::from_tuple((255.1, 203.7, 0.47));
  assert_eq!(&rgb.to_css(), "rgb(255,204,0)");
  assert_eq!(&rgb.to_rgba().to_css(), "rgba(255,204,0,1)");
  assert_eq!(&rgb.to_hex().to_css(), "#ffcc00");

  let rgb = Rgb::from_tuple((137.0, 193.0, 31.0));
  assert_eq!(&rgb.to_hsl().to_css(), "hsl(81,72%,44%)");
  assert_eq!(&rgb.to_hsla().to_css(), "hsla(81,72%,44%,1)");
}

#[test]
fn hex_from_str_test() {
  fn parse_hex(s: &str) -> Result<Hex, ParseError> {
    s.parse::<Hex>()
  }
  assert_eq!(parse_hex("#ffcc00").unwrap().to_css(), "#ffcc00");
  assert_eq!(parse_hex("#FA1CBE").unwrap().to_css(), "#fa1cbe");
  assert_eq!(parse_hex("#fc0").unwrap().to_css(), "#ffcc00");
  assert_eq!(parse_hex("fc0").unwrap().to_css(), "#ffcc00");
  assert!(parse_hex("").is_err());
  assert!(parse_hex("ffcc0g").is_err());
  assert!(parse_hex("zxc.@0").is_err());
  assert!(parse_hex("鐵").is_err());
  assert!(parse_hex("中鏽鏽").is_err());
}

#[test]
fn rgb_from_str_test() {
  fn parse_rgb(s: &str) -> Result<Rgb, ParseError> {
    s.parse::<Rgb>()
  }
  assert_eq!(parse_rgb("Rgb(134,11,251)").unwrap().to_css(), "rgb(134,11,251)");
  assert_eq!(parse_rgb("Rgb(134.9,11.1,250.55)").unwrap().to_css(), "rgb(135,11,251)");
  assert_eq!(parse_rgb("(0,0,0)").unwrap().to_css(), "rgb(0,0,0)");
  assert!(parse_rgb("").is_err());
  assert!(parse_rgb("ffcc0g").is_err());
}

#[test]
fn rgba_from_str_test() {
  fn parse_rgba(s: &str) -> Result<Rgba, ParseError> {
    s.parse::<Rgba>()
  }
  assert_eq!(parse_rgba("Rgba(134, 11, 251,0.67)").unwrap().to_css(), "rgba(134,11,251,0.67)");
  assert_eq!(
    parse_rgba("rgba( 134.9, 11.1, 250.55,0.3 )").unwrap().to_css(),
    "rgba(135,11,251,0.3)"
  );
  assert_eq!(parse_rgba("(0,0,0,0.1)").unwrap().to_css(), "rgba(0,0,0,0.1)");
  assert!(parse_rgba("asd234,234rgba").is_err());
  assert!(parse_rgba("ffcc0g").is_err());
}

#[test]
fn hsl_from_str_test() {
  fn parse_hsl(s: &str) -> Result<Hsl, ParseError> {
    s.parse::<Hsl>()
  }

  assert_eq!(
    parse_hsl("hsl(37, 0.12, 0.75%)").unwrap().to_rgb().to_css(),
    "rgb(199,193,184)".parse::<Rgb>().unwrap().to_css()
  );
}

#[test]
fn get_unit_tst() {
  fn cmp(x: f32, y: f32) -> bool {
    ((x * 100.0).round() / 100.0 - y).abs() <= std::f32::EPSILON
  }

  let rgb = Rgb::from_tuple((34.0, 12.0, 177.0));

  println!("Hue {}", rgb.get_unit(ColorUnit::Hue));
  println!("Saturation {}", rgb.get_unit(ColorUnit::Saturation));
  println!("Lightness {}", rgb.get_unit(ColorUnit::Lightness));

  assert!(cmp(rgb.get_unit(ColorUnit::Red), 34.0));
  assert!(cmp(rgb.get_unit(ColorUnit::Green), 12.0));
  assert!(cmp(rgb.get_unit(ColorUnit::Blue), 177.0));
  assert!(cmp(rgb.get_unit(ColorUnit::Hue), 248.0));
  assert!(cmp(rgb.get_unit(ColorUnit::Saturation), 0.87));
  assert!(cmp(rgb.get_unit(ColorUnit::Lightness), 0.37));
}
