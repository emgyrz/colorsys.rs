# colors_transform

[![Crates.io](https://img.shields.io/crates/v/colorsys.svg)](https://crates.io/crates/colorsys/)

A module for color conversion and mutation written in Rust. For now works with RGB(a), HSL(a) color models

[Online documentation](https://docs.rs/colorsys/0.1.2/colorsys/)

#### ...docs in progress...

##### Rgb example
```rust
 use colorsys::{Rgb, Hsl, prelude::*};
 let mut rgb1 = Rgb::from((100.0, 255.0, 17.0));
 // Rgb { r: 100.0, g: 255.0, b: 17.0, a: None }

 let green = rgb1.get_green();
 // 255.0

 rgb1.set_red(108.3);
 // Rgb { r: 108.3, g: 255.0, b: 17.0, .. }

 let mut hsl: Hsl = rgb1.into();
 // ~Hsl { h: 96.98, s: 100.0, l: 53.333, .. }

 hsl.saturate( SaturationInSpace::Hsl(-57.901) );
 // ~Hsl { h: 96.98, s: 42.099, l: 53.333, .. }

 let mut rgb2 = Rgb::from(&hsl);
 // ~Rgb { r: 124.34, g: 186.1, b: 85.9, .. }

 let rgb2tuple: (f64,f64,f64) = rgb2.as_ref().into();
 // (124.34, 186.1,85.9)

 rgb2 += Rgb::from_hex_str("#35f15b").unwrap();;
 // ~Rgb { r: 177.33, g: 255.0, b: 176.902, .. }

 rgb2.set_green(-150.0);
 assert_eq!(rgb2.get_green(), 0.0);

 rgb2.lighten(-13.123);
 // ~Rgb { r: 110.41, g: 0.0, b: 110.1, .. }

 rgb2.grayscale_simple();
 // ~Rgb { r: 55.2, g: 55.2, b: 55.2, .. }

 let css_string = rgb2.to_css_string();
 assert_eq!(css_string, "rgb(55,55,55)");
```

<!-- ##### Enjoy using! -->

### License

This module is [MIT licensed](./LICENSE).
