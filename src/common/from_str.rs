// static RGB_START: &[u8; 3] = b"rgb";
// static HSL_START: &[u8; 3] = b"rgb";
// static COMMA: u8 = b',';
// static DOT: u8 = b'.';
//
// fn is_delimiter(ch: u8) -> bool {
//   ch.is_ascii_whitespace() || ch == COMMA
// }
// fn is_val_char(ch: u8) -> bool {
//   ch.is_ascii_digit() || ch == DOT
// }
//
// fn from_bytes(bytes: &[u8]) -> Result<([f64; 4], usize), ()> {
//   let bytes_len = bytes.len();
//   if bytes_len == 0 { return Err(()) }
//   let mut buff = [0.0; 4];
//   let mut buff_len = 0;
//
//   let mut i = 0;
//   while i < bytes_len {
//     let ch = bytes[i];
//     if !is_val_char(ch) {
//       i += 1;
//       continue
//     }
//
//     let mut chi = i + 1;
//     while chi < bytes_len && is_val_char(bytes[chi]) {
//       chi += 1;
//     }
//     let s = core::str::from_utf8(&bytes[i..chi]).map_err(|_| ())?;
//     if buff_len == 4 { return Err(()) }
//     buff[buff_len] = s.parse().map_err(|_| ())?;
//     buff_len += 1;
//     i = chi + 1;
//   }
//
//   if buff_len == 0 { return Err(()) }
//
//   Ok((buff, buff_len))
// }
//
//
// #[allow(clippy::float_cmp)]
// #[cfg(test)]
// mod test {
//   use crate::common::from_str::from_bytes;
//
//   #[test]
//   fn collect_digits_from_str_test() {
//     let valid = [
//       ("10,30,40", [10.0f64, 30.0, 40.0, 0.0]),
//       ("10.5,30, 400.1, 22", [10.5, 30.0, 400.1, 22.0]),
//       ("rgb( 0,30, 400.1, 22) ", [0.0, 30.0, 400.1, 22.0]),
//       ("1.33 ", [1.33, 0.0, 0.0, 0.0]),
//     ];
//     let invalid = [
//       "",
//       "asd",
//       "rgb( 0,30, 400.1, 22, 1) "
//     ];
//
//     for (s, arr) in &valid {
//       assert_eq!(&from_bytes(s.as_bytes()).unwrap().0, arr);
//     }
//
//     for s in &invalid {
//       assert!(&from_bytes(s.as_bytes()).is_err());
//     }
//   }
// }