pub struct Hex {
  hex: String,
  num: usize,
}

impl Clone for Hex {
  fn clone(&self) -> Hex {
    Hex {
      num: self.num,
      hex: self.hex.clone(),
    }
  }
}
