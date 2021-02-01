use crate::units::{Units};


impl<'a> Into<[f64;3]> for &'a Units {
  fn into(self) -> [f64; 3] {
    [self[0],self[1],self[2]]
  }
}
impl Into<[f64;3]> for Units {
  fn into(self) -> [f64; 3] {
    [self[0],self[1],self[2]]
  }
}
impl<'a> Into<[f64;4]> for &'a Units {
  fn into(self) -> [f64; 4] {
    [self[0],self[1],self[2],self[3]]
  }
}
impl Into<[f64;4]> for Units {
  fn into(self) -> [f64; 4] {
    [self[0],self[1],self[2],self[3]]
  }
}

// impl<'a> Into<[f32;3]> for &'a Units {
//   fn into(self) -> [f32; 3] {
//     [self[0] as f32,self[1] as f32,self[2] as f32]
//   }
// }
//
// impl<'a> Into<(f64,f64,f64)> for &'a Units {
//   fn into(self) -> (f64,f64,f64) {
//     (self[0],self[1],self[2])
//   }
// }
// impl<'a> Into<(f32,f32,f32)> for &'a Units {
//   fn into(self) -> (f32,f32,f32) {
//     (self[0] as f32,self[1] as f32,self[2] as f32)
//   }
// }

