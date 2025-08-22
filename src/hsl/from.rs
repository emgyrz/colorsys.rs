use crate::{ColorAlpha, converters::*, ratio_converters::ratio_to_hsla};

use super::{Hsl, HslRatio, Rgb};

macro_rules! from_for_hsl {
  ($from_type: ty, $val: ident, $conv: block) => {
    impl From<&$from_type> for Hsl {
      fn from($val: &$from_type) -> Hsl {
        ($conv)
      }
    }
    impl From<$from_type> for Hsl {
      fn from($val: $from_type) -> Hsl {
        Hsl::from(&$val)
      }
    }
  };
}

macro_rules! from_for_hsl_all {
  ($t: ty) => {
    from_for_hsl!(($t, $t, $t), v, {
      let (h, s, l) = *v;
      Hsl::new(h as f64, s as f64, l as f64, None)
    });
    from_for_hsl!([$t; 3], v, {
      let [h, s, l] = *v;
      Hsl::new(h as f64, s as f64, l as f64, None)
    });
  };
}

macro_rules! from_for_hsl_all_with_alpha {
  ($t: ty) => {
    from_for_hsl!(($t, $t, $t, $t), v, {
      let (h, s, l, a) = *v;
      Hsl::new(h as f64, s as f64, l as f64, Some(a as f64))
    });
    from_for_hsl!([$t; 4], v, {
      let [h, s, l, a] = *v;
      Hsl::new(h as f64, s as f64, l as f64, Some(a as f64))
    });
  };
}

from_for_hsl_all!(f32);
from_for_hsl_all!(f64);
from_for_hsl_all_with_alpha!(f32);
from_for_hsl_all_with_alpha!(f64);
from_for_hsl_all!(i16);
from_for_hsl_all!(i32);
from_for_hsl_all!(i64);
from_for_hsl_all!(u16);
from_for_hsl_all!(u32);
from_for_hsl_all!(u64);

fn from_rgb(rgb: &Rgb) -> Hsl {
  let a = rgb.alpha();
  let mut hsl = Hsl::from_units(rgb_to_hsl(rgb));
  hsl.set_alpha(a);
  hsl
}

impl From<&Rgb> for Hsl {
  /// # Example
  /// ```
  /// use colorsys::{Rgb,Hsl,prelude::*};
  /// let rgb = Rgb::from(&(215.0, 231.0, 236.0));
  /// let hsl = Hsl::from(&rgb);
  /// assert_eq!(hsl.to_css_string(), "hsl(194,36%,88%)");
  /// ```
  fn from(rgb: &Rgb) -> Self {
    from_rgb(rgb)
  }
}

impl From<&mut Rgb> for Hsl {
  /// # Example
  /// ```
  /// use colorsys::{Rgb,Hsl,prelude::*};
  /// let mut rgb = Rgb::from(&(0.0, 0.0, 0.0));
  /// let hsl_string = Hsl::from(&mut rgb).to_css_string();
  /// assert_eq!(hsl_string, "hsl(0,0%,0%)");
  /// ```
  fn from(rgb: &mut Rgb) -> Self {
    from_rgb(rgb)
  }
}

impl From<Rgb> for Hsl {
  /// # Example
  /// ```
  /// use colorsys::{Rgb,Hsl,prelude::*};
  /// let rgb = Rgb::from(&(255.0, 255.0, 255.0));
  /// let hsl_string = Hsl::from(rgb).to_css_string();
  /// assert_eq!(hsl_string, "hsl(0,0%,100%)");
  /// ```
  fn from(rgb: Rgb) -> Self {
    from_rgb(&rgb)
  }
}

fn from_hsl_ratio(ratio: &HslRatio) -> Hsl {
  let ru = &ratio.units;
  let t = ratio_to_hsla(&(ru[0], ru[1], ru[2], ru.alpha.get_f64()));
  Hsl::new(t.0, t.1, t.2, Some(t.3))
}

impl From<&HslRatio> for Hsl {
  fn from(r: &HslRatio) -> Self {
    from_hsl_ratio(r)
  }
}

impl From<&mut HslRatio> for Hsl {
  fn from(r: &mut HslRatio) -> Self {
    from_hsl_ratio(r)
  }
}

impl From<HslRatio> for Hsl {
  fn from(r: HslRatio) -> Self {
    from_hsl_ratio(&r)
  }
}

//
//
//
// INTO
//

macro_rules! into_for_hsl_all {
  ($t: ty) => {
    into_for_some!(($t, $t, $t), Hsl, self, {
      let u = &self.units;
      (u[0] as $t, u[1] as $t, u[2] as $t)
    });
    into_for_some!([$t; 3], Hsl, self, {
      let u = &self.units;
      [u[0] as $t, u[1] as $t, u[2] as $t]
    });
  };
}

macro_rules! into_for_hsl_all_with_alpha {
  ($t: ty) => {
    into_for_some!(($t, $t, $t, $t), Hsl, self, {
      let u = &self.units;
      (u[0] as $t, u[1] as $t, u[2] as $t, self.units.alpha.get_f64() as $t)
    });
    into_for_some!([$t; 4], Hsl, self, {
      let u = &self.units;
      [u[0] as $t, u[1] as $t, u[2] as $t, self.units.alpha.get_f64() as $t]
    });
  };
}

into_for_hsl_all!(f32);
into_for_hsl_all!(f64);
into_for_hsl_all_with_alpha!(f32);
into_for_hsl_all_with_alpha!(f64);
into_for_hsl_all!(i16);
into_for_hsl_all!(i32);
into_for_hsl_all!(i64);
into_for_hsl_all!(u16);
into_for_hsl_all!(u32);
into_for_hsl_all!(u64);
