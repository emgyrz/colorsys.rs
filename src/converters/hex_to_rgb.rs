use err::{make_parse_err, ParseError};

use crate::err;

const HASH: u8 = b'#';


pub(crate) fn hex_to_rgb(s: &str) -> Result<[u32; 3], ParseError> {
  from_hex(s.as_bytes()).map_err(|_| make_parse_err(s, "hex"))
}

pub(crate) fn from_hex(s: &[u8]) -> Result<[u32; 3], ()> {
  let mut buff: [u8; 6] = [0; 6];
  let mut buff_len = 0;


  for b in s {
    if !b.is_ascii() || buff_len == 6 {
      return Err(());
    }

    let bl = b.to_ascii_lowercase();
    if bl == HASH { continue; }
    if bl.is_ascii_hexdigit() {
      buff[buff_len] = bl;
      buff_len += 1;
    } else {
      return Err(());
    }
  }

  if buff_len == 3 {
    buff = [buff[0], buff[0], buff[1], buff[1], buff[2], buff[2]];
  }

  let hex_str = core::str::from_utf8(&buff).map_err(|_| ())?;
  let hex_digit = u32::from_str_radix(hex_str, 16).map_err(|_| ())?;

  Ok(hex_digit_to_rgb(hex_digit))
}


fn hex_digit_to_rgb(num: u32) -> [u32; 3] {
  let r = num >> 16;
  let g = (num >> 8) & 0x00FF;
  let b = num & 0x0000_00FF;

  [r, g, b]
}

#[cfg(test)]
mod test {
  use crate::converters::hex_to_rgb::{from_hex};

  #[test]
  fn from_hex_test() {
    let valid = [
      ("000", [0u32; 3]),
      ("#000", [0; 3]),
      ("#000000", [0; 3]),
      ("fFf", [255; 3]),
      ("#fff", [255; 3]),
      ("#ffFfff", [255; 3]),
      ("#ffffff", [255; 3]),
      ("#777", [119; 3]),
      ("F7b3aA", [247, 179, 170])
    ];

    let invalid = [
      "0000",
      "#0000f221",
      "#000000a",
      "тест",
      "ffccfg",
      "",
      "Magenta"
    ];

    for (s, t) in valid.iter() {
      let rgb = from_hex(s.as_bytes()).unwrap();
      assert_eq!(&rgb, t);
    }

    for s in invalid.iter() {
      let result = from_hex(s.as_bytes());
      assert!(result.is_err());
    }
  }
}


