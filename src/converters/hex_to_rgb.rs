use err::{make_parse_err, ParseError};

use crate::{consts::RGB_UNIT_MAX, err};

const HASH: u8 = b'#';
const HEX_BYTES_BUFF_SIZE: usize = 8;
const MAX_HEX_BYTE: u8 = b'f';

pub(crate) fn hex_to_rgb(s: &str) -> Result<([u32; 3], Option<f64>), ParseError> {
  from_hex(s.as_bytes()).map_err(|_| make_parse_err(s, "hex"))
}

fn from_hex(s: &[u8]) -> Result<([u32; 3], Option<f64>), ()> {
  let mut buff = create_hex_bytes_buffer();
  let mut buff_cap = 0;

  for b in s {
    if !b.is_ascii() || buff_cap == HEX_BYTES_BUFF_SIZE {
      return Err(());
    }

    let bl = b.to_ascii_lowercase();

    if bl == HASH { continue; }

    if bl.is_ascii_hexdigit() {
      buff[buff_cap] = bl;
      buff_cap += 1;
    } else {
      return Err(());
    }
  }

  let is_short_hexa = buff_cap == 4;
  if buff_cap == 3 || is_short_hexa {
    buff = [
            buff[0], buff[0], buff[1], buff[1], buff[2], buff[2], 
            if is_short_hexa { buff[3] } else { buff[6] }, 
            if is_short_hexa { buff[3] } else { buff[7] }, 
    ];
    buff_cap = HEX_BYTES_BUFF_SIZE;
  }

  if buff_cap != 6 && buff_cap != HEX_BYTES_BUFF_SIZE {
     return Err(());
  }


  let mut alpha = None;
  if buff[6] != MAX_HEX_BYTE || buff[7] != MAX_HEX_BYTE {
    let a_hex_digit = hex_bytes_to_u32(&buff[6..8]).map_err(|_| ())? as f64;
    alpha = Some(a_hex_digit / RGB_UNIT_MAX);
  }

  let hex_digit = hex_bytes_to_u32(&buff[0..6]).map_err(|_| ())?;
  Ok((hex_digit_to_rgb(hex_digit), alpha))
}


fn create_hex_bytes_buffer() -> [u8; HEX_BYTES_BUFF_SIZE] {
  let mut buff: [u8; HEX_BYTES_BUFF_SIZE] = [0; HEX_BYTES_BUFF_SIZE];

  buff[6] = MAX_HEX_BYTE;
  buff[7] = MAX_HEX_BYTE;

  buff
}


fn hex_digit_to_rgb(num: u32) -> [u32; 3] {
  let r = num >> 16;
  let g = (num >> 8) & 0x00FF;
  let b = num & 0x0000_00FF;

  [r, g, b]
}

fn hex_bytes_to_u32(bytes: &[u8]) -> Result<u32, ()> {
  let hex_str = core::str::from_utf8(bytes).map_err(|_| ())?;
  u32::from_str_radix(hex_str, 16).map_err(|_| ())
}

#[cfg(test)]
mod test {
  use crate::{common::approx::approx, converters::hex_to_rgb::from_hex};

  #[test]
  fn from_hex_test() {
    let valid = [
      ("000", [0u32; 3]),
      ("0000", [0u32; 3]),
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
      "#000000a",
      "тест",
      "#00",
      "ffccfg",
      "ffccf",
      "",
      "Magenta"
    ];

    for (s, t) in valid.iter() {
      let rgb = from_hex(s.as_bytes()).unwrap();
      assert_eq!(&rgb.0, t);
    }

    for s in invalid.iter() {
      let result = from_hex(s.as_bytes());
      assert!(result.is_err());
    }
  }


  #[test]
  fn from_hexa_test() {
    let valid = [
      ("0003", [0u32; 3], Some(0.2)),
      ("#1473", [17, 68, 119], Some(0.2)),
      ("#11447733", [17, 68, 119], Some(0.2)),
      ("#0000f221", [0, 0, 242], Some(0.129411)),
      ("#3E8ED0ab", [62, 142, 208], Some(0.67)),
      ("000", [0u32; 3], None)
    ];

    for (s, t, a) in valid.iter() {
      let rgb = from_hex(s.as_bytes()).unwrap();
      assert_eq!(&rgb.0, t);
      if rgb.1.is_none() {
        assert!(a.is_none());
      } else {
        assert!(approx(rgb.1.unwrap(), a.unwrap(), 0.001));
      } 
    }

  }
}


