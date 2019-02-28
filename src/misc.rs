use super::{Color, ColorUnit};

pub fn get_unit<C: Color>(col: C, unit: ColorUnit) -> f32 {
  match unit {
    ColorUnit::Red => col.to_rgb().as_tuple().0,
    ColorUnit::Green => col.to_rgb().as_tuple().1,
    ColorUnit::Blue => col.to_rgb().as_tuple().2,
    ColorUnit::Hue => col.to_hsl().as_tuple().0,
    ColorUnit::Saturation => col.to_hsl().as_tuple().1,
    ColorUnit::Lightness => col.to_hsl().as_tuple().2,
  }
}
