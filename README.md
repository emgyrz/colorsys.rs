# colors_transform


[![Crates.io](https://img.shields.io/crates/v/colors_transform.svg)](https://crates.io/crates/colors-transform/)

A module for color conversion and mutation written in Rust.

[Online documentation](https://docs.rs/colors-transform/0.2.11/colors_transform/)

For now you can work with four color representation options: Rgb (Rgba), Hsl (Hsla). Each of them has a variety of methods to modify and convert. See the [Color](https://docs.rs/colors-transform/0.2.11/colors_transform/trait.Color.html) trait they implement. There are also a couple of methods for hex string color.

All values are given as f32 for more accurate calculations.

## What It Can Do

#### getters & setters
```Rust
use colors_transform::{Rgb, Color};
let rgb = Rgb::from(57.3, 12.7, 53.0);
// where tuple is ($red, $green, $blue)

let modified = rgb
  .set_red(245.0) // Rgb { r: 245.0, g: 152.0, b: 53.0 }
  .set_green(152.0) // Rgb { r: 245.0, g: 152.0, b: 53.0 }
  .set_hue(279.0); // Rgb { r: 177.80003, g: 53.00001, b: 245.0 }

let saturation = modified.get_saturation(); // 63.71429
let blue = modified.get_blue(); // 53.00001

```

#### conversion
```Rust
let hex_color = Hsl::from(315.9, 99.7, 50.0)
// where tuple is ($hue, $saturation, $lightness)
  .to_rgb() // ~Rgb { r: 254.6, g: 0.38, b: 187.24 }
  .set_saturation(33.3) // ~Rgb { r: 169.9, g: 85.04, b: 147.45 }
  .to_hsl() // Hsl { h: 315.9, s: 33.3, l: 50.0 }
  .set_alpha(0.47) // Hsl { h: 315.9, s: 99.7, l: 50.0 } // a: 0.47
  .to_rgb() // Rgb { r: 169.95749, g: 85.0425, b: 147.45502 }
  .to_css_hex_string(); // #aa5593
```

#### modification
```Rust
let rgb_tuple = (245.0,152.0,53.0);
let rgb = Rgb::from_tuple(&rgb_tuple)
  .lighten(21.0) // Rgb { r: 250.05188, g: 204.03442, b: 155.04813 }
  .saturate( 3.9999 ); // Rgb { r: 252.14981, g: 204.1, b: 152.9502 }
  .invert(); // Rgb { r: 2.8501892, g: 50.899994, b: 102.049805 }
```

#### parsing from string & css string representation
```Rust
let hsl: Hsl = "hsl(359,12%,71)".parse().unwrap();
// Hsl { h: 359.0, s: 12.0, l: 71.0 }

let rgb1 = "rgb(12,13,14)"
  .parse::<Rgb>()
  .unwrap()
  .adjust_color( RgbUnit::Green, 139.7 );
// Rgb { r: 12.0, g: 152.7, b: 14.0 }

let rgb2 = Rgb::from_hex_str("#fc0").unwrap();
// Rgb { r: 255.0, g: 204.0, b: 0.0 }

let rgb_str = rgb1.to_css_string();
// rgb(12,153,14)

let hsl_str = rgb2.to_hsl().to_css_string();
// "hsl(48,100%,50%)"
```

As you see it is completely chainable.


## Color unit ranges
All color units is f32. Here are their ranges:
 - red - 0.0 .. 255.0
 - green - 0.0 .. 255.0
 - blue - 0.0 .. 255.0
 - hue - 0.0 .. 360.0
 - saturation - 0.0 .. 100.0
 - lightness - 0.0 .. 100.0
 - alpha - 0.0 .. 1.0

If you specify a value that does not fit within these ranges, they are replaced with a minimum or maximum value.

##### Enjoy using!

### License

This module is [MIT licensed](./LICENSE).
