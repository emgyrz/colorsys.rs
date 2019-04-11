pub trait ColorAlpha {
  fn get_alpha(&self) -> f64;
  fn set_alpha(&mut self, val: f64);
  fn opacify(&mut self, val: f64);
}
