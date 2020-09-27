use crate::converters::*;
use crate::rgb::RgbRatio;
use crate::{
  ratio_converters::ratio_to_rgba, ColorAlpha, ColorTuple, Hsl, Rgb,
};

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
    from_for_rgb!(($t, $t, $t, $t), v, {
      let (r, g, b, a) = *v;
      Rgb::new(r as f64, g as f64, b as f64, Some(a as f64))
    });
    from_for_rgb!([$t; 3], v, {
      let [r, g, b] = *v;
      Rgb::new(r as f64, g as f64, b as f64, None)
    });
    from_for_rgb!([$t; 4], v, {
      let [r, g, b, a] = *v;
      Rgb::new(r as f64, g as f64, b as f64, Some(a as f64))
    });
  };
}

from_for_rgb_all!(f32);
from_for_rgb_all!(f64);
from_for_rgb_all!(i16);
from_for_rgb_all!(i32);
from_for_rgb_all!(i64);
from_for_rgb_all!(u8);
from_for_rgb_all!(u16);
from_for_rgb_all!(u32);
from_for_rgb_all!(u64);

fn from_hsl(hsl: &Hsl) -> Rgb {
  let a = hsl.get_alpha();
  let tuple: ColorTuple = hsl.into();
  let mut rgb = Rgb::from(hsl_to_rgb(&tuple));
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
  let t = ratio_to_rgba(&(ratio.r, ratio.g, ratio.b, ratio.a));
  Rgb { r: t.0, g: t.1, b: t.2, a: Some(t.3) }
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
      let Rgb { r, g, b, .. } = *self;
      (r as $t, g as $t, b as $t)
    });
    into_for_some!(($t, $t, $t, $t), Rgb, self, {
      let Rgb { r, g, b, .. } = *self;
      (r as $t, g as $t, b as $t, self.get_alpha() as $t)
    });
    into_for_some!([$t; 3], Rgb, self, {
      let Rgb { r, g, b, .. } = *self;
      [r as $t, g as $t, b as $t]
    });
    into_for_some!([$t; 4], Rgb, self, {
      let Rgb { r, g, b, .. } = *self;
      [r as $t, g as $t, b as $t, self.get_alpha() as $t]
    });
  };
}

into_for_rgb_all!(f32);
into_for_rgb_all!(f64);
into_for_rgb_all!(i16);
into_for_rgb_all!(i32);
into_for_rgb_all!(i64);
into_for_rgb_all!(u8);
into_for_rgb_all!(u16);
into_for_rgb_all!(u32);
into_for_rgb_all!(u64);
