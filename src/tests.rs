use super::converters::{
  as_rounded_hsl_tuple, as_rounded_rgb_tuple, hsl_to_rgb, ratio_as_percent, rgb_to_hex, rgb_to_hsl,
};
use super::{Color, Hex, Hsl, Hsla, Rgb, RgbColor, Rgba};

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
