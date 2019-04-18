use crate::converters::*;
use crate::{ColorAlpha, ColorTuple, ColorTupleA, Hsl, Rgb};

impl std::convert::From<&ColorTuple> for Rgb {
  fn from(t: &ColorTuple) -> Rgb {
    let (h, s, l) = *t;
    Rgb::new(h, s, l, None)
  }
}

impl std::convert::From<ColorTuple> for Rgb {
  fn from(t: ColorTuple) -> Rgb {
    Rgb::from(&t)
  }
}

impl From<&ColorTupleA> for Rgb {
  fn from(t: &ColorTupleA) -> Rgb {
    let (h, s, l, a) = *t;
    Rgb::new(h, s, l, Some(a))
  }
}

impl From<ColorTupleA> for Rgb {
  fn from(t: ColorTupleA) -> Rgb {
    Rgb::from(&t)
  }
}

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

//
//
//
// INTO
//

impl<'a> Into<ColorTuple> for &'a Rgb {
  fn into(self) -> ColorTuple {
    let Rgb { r, g, b, .. } = *self;
    (r, g, b)
  }
}
impl<'a> Into<ColorTuple> for &'a mut Rgb {
  fn into(self) -> ColorTuple {
    let Rgb { r, g, b, .. } = *self;
    (r, g, b)
  }
}
impl Into<ColorTuple> for Rgb {
  fn into(self) -> ColorTuple {
    self.as_ref().into()
  }
}

impl<'a> Into<ColorTupleA> for &'a Rgb {
  fn into(self) -> ColorTupleA {
    let Rgb { r, g, b, .. } = *self;
    (r, g, b, self.get_alpha())
  }
}
impl<'a> Into<ColorTupleA> for &'a mut Rgb {
  fn into(self) -> ColorTupleA {
    let Rgb { r, g, b, .. } = *self;
    (r, g, b, self.get_alpha())
  }
}

impl Into<ColorTupleA> for Rgb {
  fn into(self) -> ColorTupleA {
    self.as_ref().into()
  }
}
