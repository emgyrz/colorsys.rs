use crate::Rgb;

/// Predefined set of 256 colors to use with ANSI escape sequences,
/// e.g. in terminal emulators
///
/// # Example
/// ```
/// use colorsys::{Ansi256, Rgb};
///
/// let rgb = Rgb::from_hex_str("#875fff").unwrap();
/// let ansi256: Ansi256 = rgb.into();
/// assert_eq!(ansi256.code(),99);
///
/// let green_yellow = Ansi256::new(154);
/// let rgb2 = green_yellow.as_rgb();
/// assert_eq!(rgb2.to_hex_string(), "#afff00");
///
/// let txt = format!(
///   "\x1b[38;5;{ansi_code}m{text}\x1b[0m",
///   ansi_code=green_yellow.code(),
///   text="my colored text"
/// );
/// println!("{}", txt);
///
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Ansi256(pub(crate) u8);

impl Ansi256 {
  pub fn new(ansi_code: u8) -> Self {
    Ansi256(ansi_code)
  }
  pub fn code(&self) -> u8 {
    self.0
  }
  pub fn set_code(&mut self, v: u8) {
    self.0 = v;
  }

  pub fn as_rgb(&self) -> Rgb {
    (*self).into()
  }
}


impl From<&Rgb> for Ansi256 {
  fn from(rgb: &Rgb) -> Self {
    let [r, g, b]: [u8; 3] = rgb.into();
    let red = if r < 75 { 0 } else { (r - 35) / 40 };
    let green = if g < 75 { 0 } else { (g - 35) / 40 };
    let blue = if b < 75 { 0 } else { (b - 35) / 40 };
    Ansi256(red * 6 * 6 + green * 6 + blue + 16)
  }
}


impl From<Rgb> for Ansi256 {
  fn from(rgb: Rgb) -> Self {
    rgb.as_ref().into()
  }
}


impl From<Ansi256> for Rgb {
  fn from(ansi256: Ansi256) -> Self {
    let code = ansi256.0;
    if code < 16 {
      let mut base = code;
      let mut mul = 128;
      if code == 7 { mul = 192; } else if code == 8 { base = 7; } else if code > 8 { mul = 255; }
      let r = (base & 1) * mul;
      let g = ((base & 2) >> 1) * mul;
      let b = ((base & 4) >> 2) * mul;
      Rgb::from((r, g, b))

    } else if code > 231 {
      let gray = (code - 232) * 10 + 8;
      Rgb::from((gray, gray, gray))

    } else {
      let b = (code - 16) % 6;
      let b = if b == 0 { 0 } else { b * 40 + 55 };

      let g = ((code - 16) / 6) % 6;
      let g = if g == 0 { 0 } else { g * 40 + 55 };

      let r = (code - 16) / 36;
      let r = if r == 0 { 0 } else { r * 40 + 55 };

      Rgb::from((r, g, b))
    }
  }
}


#[cfg(test)]
mod test {
  use crate::ansi::Ansi256;
  use crate::Rgb;

  #[test]
  fn rgb_to_ansi_test() {
    let test_data = [
      ([95u8,0,175], 55u8),
      ([135,95,255], 99),
      ([135,255,255], 123),
      ([215,175,95], 179),
      ([255,215,255], 225),
    ];
    for (rgb_arr, ansi_code) in &test_data {
      let ansi: Ansi256 = Rgb::from(rgb_arr).as_ref().into();
      assert_eq!(ansi.code(), *ansi_code);
    }
  }

  #[test]
  fn ansi_to_rgb_test() {
    let test_data = [
      (9u8, [255u8, 0, 0]),
      (211, [255,135,175]),
      (187, [215,215,175]),
      (171, [215,95,255]),
      (77, [95,215,95]),
      (0, [0,0,0]),
      (13, [255,0,255]),
      (249, [178,178,178]),
    ];
    for (ansi_code, rgb_arr) in &test_data {
      let rgb: Rgb = Ansi256(*ansi_code).into();
      let arr: [u8;3] = rgb.into();
      assert_eq!(&arr, rgb_arr);
    }

  }
}

