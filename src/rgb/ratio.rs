#[derive(Clone)]
pub struct RgbRatio {
  pub(super) r: f64,
  pub(super) g: f64,
  pub(super) b: f64,
  pub(super) a: f64,
}

impl RgbRatio {}

impl AsRef<RgbRatio> for RgbRatio {
  fn as_ref(&self) -> &RgbRatio {
    &self
  }
}

impl<'a> Into<[f64; 4]> for &'a RgbRatio {
  fn into(self) -> [f64; 4] {
    let RgbRatio { r, g, b, a } = *self;
    [r, g, b, a]
  }
}

impl<'a> Into<[f64; 4]> for &'a mut RgbRatio {
  fn into(self) -> [f64; 4] {
    let RgbRatio { r, g, b, a } = *self;
    [r, g, b, a]
  }
}

impl Into<[f64; 4]> for RgbRatio {
  fn into(self) -> [f64; 4] {
    self.as_ref().into()
  }
}

impl<'a> Into<[f64; 3]> for &'a RgbRatio {
  fn into(self) -> [f64; 3] {
    let RgbRatio { r, g, b, .. } = *self;
    [r, g, b]
  }
}

impl<'a> Into<[f64; 3]> for &'a mut RgbRatio {
  fn into(self) -> [f64; 3] {
    let RgbRatio { r, g, b, .. } = *self;
    [r, g, b]
  }
}

impl Into<[f64; 3]> for RgbRatio {
  fn into(self) -> [f64; 3] {
    self.as_ref().into()
  }
}
