use super::converters::{as_rounded_hsl_tuple, as_rounded_rgb_tuple, hsl_to_rgb};
use super::{Color, GrayScaleMethod, Hsl, ParseError, Rgb, RgbUnit};

// #[test]
// fn speed_test() {
//   use std::time::Duration;
//   let count = 10000;
//   let start = std::time::Instant::now();
//   let rgb = Rgb::from_tuple((255.0, 255.0, 255.0));
//   let mut tmp = Vec::new();
//   for _ in 0..count {
//     tmp.push(rgb.to_hsl());
//   }

//   println!("Elapsed {:?} for {} times", start.elapsed(), tmp.len());
// }

#[test]
fn hsl_to_rgb_test() {
  let asserts = [
    ((200.0, 100.0, 30.0), (0, 102, 153)),
    ((192.0, 67.0, 28.0), (24, 100, 119)),
    ((48.0, 70.0, 50.0), (217, 181, 38)),
    ((359.0, 33.0, 77.0), (216, 177, 178)),
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
    assert_eq!(as_rounded_hsl_tuple(&Rgb::from_tuple(&a.0).to_hsl().as_tuple()), a.1);
  });
}

#[test]
fn lighten_darken_test() {
  let asserts = [
    ((30.0, 108.0, 77.0), 20.0, (52, 188, 134)),
    ((30.0, 108.0, 77.0), 90.0, (255, 255, 255)),
    ((30.0, 108.0, 77.0), -20.0, (8, 28, 20)),
    ((0.0, 0.0, 0.0), 50.0, (128, 128, 128)),
    ((0.0, 0.0, 0.0), -50.0, (0, 0, 0)),
    ((0.0, 0.0, 0.0), 300.5, (255, 255, 255)),
  ];

  for a in asserts.iter() {
    let (origin, amt, result) = *a;
    let rgb = Rgb::from_tuple(&origin);
    let mutated = rgb.lighten(amt);
    assert_eq!(as_rounded_rgb_tuple(&mutated.as_tuple()), result);
  }
}

#[test]
fn saturate_desaturate_test() {
  let asserts = [
    ((120.0, 30.0, 90.0), 20.0, (135, 15, 95)),
    ((120.0, 30.0, 90.0), -20.0, (105, 45, 85)),
    ((13.0, 55.0, 137.0), 30.0, (0, 51, 150)),
  ];

  for a in asserts.iter() {
    let (origin, amt, result) = *a;
    let rgb = Rgb::from_tuple(&origin);
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
    let rgb = Rgb::from_tuple(&origin);
    let mutated = rgb.adjust_hue(amt);
    assert_eq!(as_rounded_rgb_tuple(&mutated.as_tuple()), result);
  }
}

#[test]
fn adjust_color_test() {
  let rgb = Rgb::from_tuple(&(24.0, 91.0, 203.0));
  let rgb = rgb.adjust_color(RgbUnit::Red, 55.0);
  assert_eq!(rgb.as_tuple(), (79.0, 91.0, 203.0));
  let asserts = [
    ((24.0, 90.0, 20.0), RgbUnit::Red, 63.0, (160, 42, 5)),
    ((324.0, 77.0, 52.0), RgbUnit::Green, 122.0, (227, 160, 151)),
    ((195.0, 31.0, 87.0), RgbUnit::Blue, -39.0, (212, 227, 193)),
  ];

  for a in asserts.iter() {
    let (origin, col_name, col_val, result) = *a;
    let hsl = Hsl::from_tuple(&origin);
    let mutated = hsl.adjust_color(col_name, col_val);
    assert_eq!(as_rounded_rgb_tuple(&mutated.to_rgb().as_tuple()), result);
  }
}

#[test]
fn to_css_string_test() {
  let rgb = Rgb::from_tuple(&(255.1, 203.7, 0.47));
  assert_eq!(&rgb.to_css_string(), "rgb(255,204,0)");
  assert_eq!(&rgb.to_css_hex_string(), "#ffcc00");

  let rgb = Rgb::from_tuple(&(137.0, 193.0, 31.0));
  assert_eq!(&rgb.to_hsl().to_css_string(), "hsl(81,72%,44%)");
}

#[test]
fn hex_from_str_test() {
  fn parse_hex(s: &str) -> Result<Rgb, ParseError> {
    Rgb::from_hex_str(s)
  }
  assert_eq!(parse_hex("#ffcc00").unwrap().to_css_hex_string(), "#ffcc00");
  assert_eq!(parse_hex("#FA1CBE").unwrap().to_css_hex_string(), "#fa1cbe");
  assert_eq!(parse_hex("#fc0").unwrap().to_css_hex_string(), "#ffcc00");
  assert_eq!(parse_hex("fc0").unwrap().to_css_hex_string(), "#ffcc00");
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
  assert_eq!(parse_rgb("Rgb(134,11,251)").unwrap().to_css_string(), "rgb(134,11,251)");
  assert_eq!(parse_rgb("Rgb(134.9,11.1,250.55)").unwrap().to_css_string(), "rgb(135,11,251)");
  assert_eq!(parse_rgb("(0,0,0)").unwrap().to_css_string(), "rgb(0,0,0)");
  assert!(parse_rgb("").is_err());
  assert!(parse_rgb("ffcc0g").is_err());
}

#[test]
fn hsl_from_str_test() {
  fn parse_hsl(s: &str) -> Result<Hsl, ParseError> {
    s.parse::<Hsl>()
  }

  assert_eq!(
    parse_hsl("hsl(37, 12, 75%)").unwrap().to_rgb().to_css_string(),
    "rgb(199,193,184)".parse::<Rgb>().unwrap().to_css_string()
  );
}

#[test]
fn get_unit_tst() {
  let rgb = Rgb::from_tuple(&(34.0, 12.0, 177.0));

  fn cmp(x: f32, y: f32) -> bool {
    (x.round() - y).abs() <= std::f32::EPSILON
  }

  assert!(cmp(rgb.get_red(), 34.0));
  assert!(cmp(rgb.get_green(), 12.0));
  assert!(cmp(rgb.get_blue(), 177.0));
  assert!(cmp(rgb.get_hue(), 248.0));
  assert!(cmp(rgb.get_saturation(), 87.0));
  assert!(cmp(rgb.get_lightness(), 37.0));
}

#[test]
fn eq_test() {
  let rgb = Rgb::from_tuple(&(34.0, 12.0, 177.0));
  let transformed_rgb = rgb.to_hsl().to_rgb();
  assert!(rgb.get_red() - transformed_rgb.get_red() < std::f32::EPSILON);
  // assert_eq!(rgb, transformed_rgb);
}

#[test]
fn invert_tst() {
  let rgb = Rgb::from(120.0, 200.0, 12.0);
  assert_eq!(rgb.invert().as_tuple(), (135.0, 55.0, 243.0));

  let hsl = Hsl::from(120.0, 20.0, 72.0);
  assert_eq!(hsl.invert().as_tuple(), (300.0, 20.0, 72.0));
}

#[test]
fn grayscale() {
  static PRECISION: f32 = 0.0001;
  fn cmp(x: f32, y: f32) -> bool {
    (x.round() - y).abs() <= PRECISION
  }

  let rgb = Rgb::from(60.0, 184.0, 120.0);
  let grayscaled_rgb = rgb.grayscale(GrayScaleMethod::AverageProminent);
  let grayscaled_from_hsl = rgb.to_hsl().grayscale().to_rgb();
  let t1 = grayscaled_rgb.as_tuple();
  let t2 = grayscaled_from_hsl.as_tuple();
  assert!(cmp(t1.0, t2.0));
  assert!(cmp(t1.1, t2.1));
  assert!(cmp(t1.2, t2.2));
}
