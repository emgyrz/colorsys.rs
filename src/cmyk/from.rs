use crate::cmyk::CmykRatio;
use crate::converters::{cmyk_to_rgb, rgb_to_cmyk};
use crate::{Cmyk, Rgb};

fn cmyk_from_ratio(r: &CmykRatio) -> Cmyk {
  let mut u = r.units.clone();
  for v in &mut u.list {
    v.turn_into_percent();
  }
  Cmyk::from_units(u)
}

impl From<&CmykRatio> for Cmyk {
  fn from(r: &CmykRatio) -> Self {
    cmyk_from_ratio(r)
  }
}

impl From<&mut CmykRatio> for Cmyk {
  fn from(r: &mut CmykRatio) -> Self {
    cmyk_from_ratio(r)
  }
}

impl From<CmykRatio> for Cmyk {
  fn from(r: CmykRatio) -> Self {
    cmyk_from_ratio(&r)
  }
}


fn cmyk_to_ratio(cmyk: &Cmyk) -> CmykRatio {
  CmykRatio::from_units(cmyk.units.as_ratio())
}

impl From<&Cmyk> for CmykRatio {
  fn from(r: &Cmyk) -> Self {
    cmyk_to_ratio(r)
  }
}

impl From<&mut Cmyk> for CmykRatio {
  fn from(r: &mut Cmyk) -> Self {
    cmyk_to_ratio(r)
  }
}

impl From<Cmyk> for CmykRatio {
  fn from(r: Cmyk) -> Self {
    cmyk_to_ratio(&r)
  }
}


impl From<&mut Rgb> for Cmyk {
  fn from(rgb: &mut Rgb) -> Self {
    rgb_to_cmyk(rgb)
  }
}

impl From<&Rgb> for Cmyk {
  fn from(rgb: &Rgb) -> Self {
    rgb_to_cmyk(rgb)
  }
}

impl From<Rgb> for Cmyk {
  fn from(rgb: Rgb) -> Self {
    rgb_to_cmyk(&rgb)
  }
}


impl From<&mut Cmyk> for Rgb {
  fn from(cmyk: &mut Cmyk) -> Self {
    cmyk_to_rgb(cmyk)
  }
}

impl From<&Cmyk> for Rgb {
  fn from(cmyk: &Cmyk) -> Self {
    cmyk_to_rgb(cmyk)
  }
}

impl From<Cmyk> for Rgb {
  fn from(cmyk: Cmyk) -> Self {
    cmyk_to_rgb(&cmyk)
  }
}


impl From<[f64; 4]> for Cmyk {
  fn from(a: [f64; 4]) -> Self {
    Cmyk::new(a[0], a[1], a[2], a[3], None)
  }
}

impl From<&[f64; 4]> for Cmyk {
  fn from(a: &[f64; 4]) -> Self {
    Cmyk::new(a[0], a[1], a[2], a[3], None)
  }
}


impl Into<[f64; 4]> for Cmyk {
  fn into(self: Cmyk) -> [f64; 4] {
    self.units.into()
  }
}

impl Into<[f64; 4]> for &Cmyk {
  fn into(self) -> [f64; 4] {
    self.units.clone().into()
  }
}
