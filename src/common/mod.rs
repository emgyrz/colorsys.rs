use std::time::{SystemTime, UNIX_EPOCH};

pub mod approx;
mod hsv_hsl_from_str;
mod iter;
pub use hsv_hsl_from_str::hsl_hsv_from_str;
pub use iter::ColorIter;

pub enum Hs {
  Hsv,
  Hsl,
}

pub fn simple_rand(max: f64) -> f64 {
  let num = vec![1, 2, 3];
  let address = &num as *const Vec<i32>;
  let num = f64::from((address as i32).abs());

  let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
  (f64::from(nanos) * num) % max
}
