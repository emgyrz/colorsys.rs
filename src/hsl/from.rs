use super::{Hsl, Rgb};
use crate::{converters::*, ColorAlpha, ColorTuple, ColorTupleA};

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
    from_for_hsl!(($t, $t, $t, $t), v, {
      let (h, s, l, a) = *v;
      Hsl::new(h as f64, s as f64, l as f64, Some(a as f64))
    });
    from_for_hsl!([$t; 3], v, {
      let [h, s, l] = *v;
      Hsl::new(h as f64, s as f64, l as f64, None)
    });
    from_for_hsl!([$t; 4], v, {
      let [h, s, l, a] = *v;
      Hsl::new(h as f64, s as f64, l as f64, Some(a as f64))
    });
  };
}

from_for_hsl_all!(f32);
from_for_hsl_all!(f64);
from_for_hsl_all!(i16);
from_for_hsl_all!(i32);
from_for_hsl_all!(i64);
from_for_hsl_all!(u16);
from_for_hsl_all!(u32);
from_for_hsl_all!(u64);

fn from_rgb(rgb: &Rgb) -> Hsl {
  let a = rgb.get_alpha();
  let tuple: ColorTuple = rgb.into();
  let mut hsl = Hsl::from(rgb_to_hsl(&tuple));
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

//
//
//
// INTO
//
macro_rules! into_for_hsl {
  ($into: ty, $sel: ident, $conv: block) => {
    impl<'a> Into<$into> for &'a Hsl {
      fn into($sel) -> $into {
        ($conv)
      }
    }

    impl<'a> Into<$into> for &'a mut Hsl {
      fn into($sel) -> $into {
        ($conv)
      }
    }

    impl Into<$into> for Hsl {
      fn into($sel) -> $into {
        $sel.as_ref().into()
      }
    }
  };
}

macro_rules! into_for_hsl_all {
  ($t: ty) => {
    into_for_hsl!(($t, $t, $t), self, {
      let Hsl { h, s, l, .. } = *self;
      (h as $t, s as $t, l as $t)
    });
    into_for_hsl!(($t, $t, $t, $t), self, {
      let Hsl { h, s, l, .. } = *self;
      (h as $t, s as $t, l as $t, self.get_alpha() as $t)
    });
    into_for_hsl!([$t; 3], self, {
      let Hsl { h, s, l, .. } = *self;
      [h as $t, s as $t, l as $t]
    });
    into_for_hsl!([$t; 4], self, {
      let Hsl { h, s, l, .. } = *self;
      [h as $t, s as $t, l as $t, self.get_alpha() as $t]
    });
  };
}

into_for_hsl_all!(f32);
into_for_hsl_all!(f64);
into_for_hsl_all!(i16);
into_for_hsl_all!(i32);
into_for_hsl_all!(i64);
into_for_hsl_all!(u16);
into_for_hsl_all!(u32);
into_for_hsl_all!(u64);
