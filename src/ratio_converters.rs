use super::consts::{HUE_MAX,PERCENT_MAX,RGB_UNIT_MAX};
use super::{ColorTuple,ColorTupleA};
use super::normalize::{normalize_ratio,normalize_rgb_unit};


pub fn rgb_to_ratio( t: &ColorTuple) -> ColorTuple {
  (
    normalize_ratio( t.0 / RGB_UNIT_MAX ),
    normalize_ratio( t.1 / RGB_UNIT_MAX ),
    normalize_ratio( t.2 / RGB_UNIT_MAX ),
  )
}

pub fn rgba_to_ratio( t: &ColorTupleA) -> ColorTupleA {
  (
    normalize_ratio( t.0 / RGB_UNIT_MAX ),
    normalize_ratio( t.1 / RGB_UNIT_MAX ),
    normalize_ratio( t.2 / RGB_UNIT_MAX ),
    normalize_ratio( t.3 ),
  )
}
pub fn ratio_to_rgb( t: &ColorTuple) -> ColorTuple {
  (
    normalize_rgb_unit( t.0 * RGB_UNIT_MAX ),
    normalize_rgb_unit( t.1 * RGB_UNIT_MAX ),
    normalize_rgb_unit( t.2 * RGB_UNIT_MAX ),
  )
}

pub fn ratio_to_rgba( t: &ColorTupleA) -> ColorTupleA {
  (
    normalize_rgb_unit( t.0 * RGB_UNIT_MAX ),
    normalize_rgb_unit( t.1 * RGB_UNIT_MAX ),
    normalize_rgb_unit( t.2 * RGB_UNIT_MAX ),
    normalize_ratio( t.3 ),
  )
}



