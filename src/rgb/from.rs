use crate::{
  ColorAlpha, Hsl, ratio_converters::ratio_to_rgba, Rgb,
};
use crate::converters::*;
use crate::rgb::RgbRatio;

macro_rules! from_for_rgb {
  ($from_type: ty, $val: ident, $conv: block) => {
    impl From<&$from_type> for Rgb {
      fn from($val: &$from_type) -> Rgb {
        ($conv)
      }
    }
    impl From<$from_type> for Rgb {
      fn from($val: $from_type) -> Rgb {
        Rgb::from(&$val)
      }
    }
  };
}

macro_rules! from_for_rgb_all {
  ($t: ty) => {
    from_for_rgb!(($t, $t, $t), v, {
      let (r, g, b) = *v;
      Rgb::new(r as f64, g as f64, b as f64, None)
    });
    from_for_rgb!([$t; 3], v, {
      let [r, g, b] = *v;
      Rgb::new(r as f64, g as f64, b as f64, None)
    });
  };
}

macro_rules! from_for_rgb_all_with_alpha {
  ($t: ty) => {
    from_for_rgb!(($t, $t, $t, $t), v, {
      let (r, g, b, a) = *v;
      Rgb::new(r as f64, g as f64, b as f64, Some(a as f64))
    });
    from_for_rgb!([$t; 4], v, {
      let [r, g, b, a] = *v;
      Rgb::new(r as f64, g as f64, b as f64, Some(a as f64))
    });
  };
}

from_for_rgb_all!(f32);
from_for_rgb_all!(f64);
from_for_rgb_all_with_alpha!(f32);
from_for_rgb_all_with_alpha!(f64);
from_for_rgb_all!(i16);
from_for_rgb_all!(i32);
from_for_rgb_all!(i64);
from_for_rgb_all!(u8);
from_for_rgb_all!(u16);
from_for_rgb_all!(u32);
from_for_rgb_all!(u64);

fn from_hsl(hsl: &Hsl) -> Rgb {
  let a = hsl.alpha();
  let mut rgb = Rgb::from_units(hsl_to_rgb(hsl));
  rgb.set_alpha(a);
  rgb
}

impl From<&Hsl> for Rgb {
  /// # Example
  /// ```
  /// use colorsys::{Rgb,Hsl,prelude::*};
  /// let hsl = Hsl::from(&(48.0, 70.0, 50.0));
  /// let rgb: Rgb = Rgb::from(&hsl);
  /// assert_eq!(rgb.to_css_string(), "rgb(217,181,38)");
  /// ```
  fn from(hsl: &Hsl) -> Self {
    from_hsl(hsl)
  }
}

impl From<&mut Hsl> for Rgb {
  /// # Example
  /// ```
  /// use colorsys::{Rgb,Hsl,prelude::*};
  /// let mut hsl = Hsl::from(&(359.0, 33.0, 77.0));
  /// let rgb_string = Rgb::from(&mut hsl).to_css_string();
  /// assert_eq!(rgb_string, "rgb(216,177,178)");
  /// ```
  fn from(hsl: &mut Hsl) -> Self {
    from_hsl(hsl)
  }
}

impl From<Hsl> for Rgb {
  /// # Example
  /// ```
  /// use colorsys::{Rgb,Hsl,prelude::*};
  /// let hsl = Hsl::from(&(192.0, 67.0, 28.0));
  /// let rgb_string = Rgb::from(hsl).to_css_string();
  /// assert_eq!(rgb_string, "rgb(24,100,119)");
  /// ```
  fn from(hsl: Hsl) -> Self {
    from_hsl(&hsl)
  }
}

fn from_rgb_ratio(ratio: &RgbRatio) -> Rgb {
  let ru = &ratio.units;
  let t = ratio_to_rgba(&(ru[0], ru[1], ru[2], ru.alpha.get_f64()));
  Rgb::new(t.0, t.1, t.2, Some(t.3))
}

impl From<&RgbRatio> for Rgb {
  fn from(r: &RgbRatio) -> Self {
    from_rgb_ratio(r)
  }
}

impl From<&mut RgbRatio> for Rgb {
  fn from(r: &mut RgbRatio) -> Self {
    from_rgb_ratio(r)
  }
}

impl From<RgbRatio> for Rgb {
  fn from(r: RgbRatio) -> Self {
    from_rgb_ratio(&r)
  }
}

//
//
//
// INTO
//

into_for_some!(RgbRatio, Rgb, self, { self.as_ratio() });

macro_rules! into_for_rgb_all {
  ($t: ty) => {
    into_for_some!(($t, $t, $t), Rgb, self, {
      let u = &self.units;
      (u[0] as $t, u[1] as $t, u[2] as $t)
    });
    into_for_some!([$t; 3], Rgb, self, {
      let u = &self.units;
      [u[0] as $t, u[1] as $t, u[2] as $t]
    });
  };
}

macro_rules! into_for_rgb_all_with_alpha {
  ($t: ty) => {
    into_for_some!(($t, $t, $t, $t), Rgb, self, {
      let u = &self.units;
      (u[0] as $t, u[1] as $t, u[2] as $t, self.units.alpha.get_f64() as $t)
    });
    into_for_some!([$t; 4], Rgb, self, {
      let u = &self.units;
      [u[0] as $t, u[1] as $t, u[2] as $t, self.units.alpha.get_f64() as $t]
    });
  };
}

into_for_rgb_all!(f32);
into_for_rgb_all!(f64);
into_for_rgb_all_with_alpha!(f32);
into_for_rgb_all_with_alpha!(f64);
into_for_rgb_all!(i16);
into_for_rgb_all!(i32);
into_for_rgb_all!(i64);
into_for_rgb_all!(u8);
into_for_rgb_all!(u16);
into_for_rgb_all!(u32);
into_for_rgb_all!(u64);
