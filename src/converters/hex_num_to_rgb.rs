use crate::ColorTuple;

pub fn hex_num_to_rgb(num: usize) -> ColorTuple {
  let r = (num >> 16) as f32;
  let g = ((num >> 8) & 0x00FF) as f32;
  let b = (num & 0x0000_00FF) as f32;

  (r, g, b)
}
