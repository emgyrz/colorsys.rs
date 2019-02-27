mod color;
mod converters;
mod error;
mod from_str;
mod tests;

pub use color::{Color, GetAlpha, Hex, Hsl, Rgb, Rgba, SetAlpha, SetHsl, SetRgb};

pub use error::ParseError;
