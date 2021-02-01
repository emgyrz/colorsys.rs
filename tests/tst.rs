#[allow(unused_imports)]
use colorsys::{prelude::*, Hsl, Rgb};

#[test]
fn for_docs() {
  use colorsys::{Rgb, Hsl};

  let rbga_tuple = (57.3, 12.7, 53.0, 0.33);
  let rgba = Rgb::from(&rbga_tuple);
  let hsla: Hsl = rgba.as_ref().into();
// ~Hsl { h: 305.78, s: 63.71, l: 13.73, a: 0.33 }

  let rgb_arr: [u8; 3] = Rgb::from(&hsla).into();
// ~[57, 13, 53]

  let hsla_tuple: (f64,f64,f64,f64) = Hsl::from( Rgb::from(rgb_arr) ).into();
// ~Hsl { h: 305.78, s: 63.71, l: 13.73, a: 1 }

  let hex: String = rgba.to_hex_string();
// #390d35


// From/Into

  let rgb1 = Rgb::from_hex_str("37ea4c").unwrap();

  let rgb2 = Rgb::from(
    Into::<[f32; 4]>::into(Rgb::from(
      Into::<[u16; 3]>::into(
        Rgb::from(
          Into::<(i32,i32,i32)>::into(
            Rgb::from(
              Into::<[i64; 3]>::into(&rgb1)
            )
          )
        )
      )
    ))
  );

  assert_eq!(rgb1, rgb2);
  //
// Ratio
//
  use colorsys::{RgbRatio, ApproxEq};
  let blue = Rgb::from([34, 111, 235]);

  let ratio: [f32; 4] = blue.as_ratio().into();
// ~[0.133, 0.435, 0.922, 1.0]

  let converted: Rgb = RgbRatio::from(&ratio).into();
  assert!(blue.approx_eq_clarify(&converted, 0.0001));
}
