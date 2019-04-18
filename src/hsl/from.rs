use super::{Hsl, Rgb};
use crate::{converters::*, ColorAlpha, ColorTuple, ColorTupleA};

impl From<&ColorTuple> for Hsl {
  fn from(t: &ColorTuple) -> Hsl {
    let (h, s, l) = *t;
    Hsl::new(h, s, l, None)
  }
}

impl From<ColorTuple> for Hsl {
  fn from(t: ColorTuple) -> Hsl {
    Hsl::from(&t)
  }
}

impl From<&ColorTupleA> for Hsl {
  fn from(t: &ColorTupleA) -> Hsl {
    let (h, s, l, a) = *t;
    Hsl::new(h, s, l, Some(a))
  }
}

impl From<ColorTupleA> for Hsl {
  fn from(t: ColorTupleA) -> Hsl {
    Hsl::from(&t)
  }
}

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
  /// assert_eq!(hsl.to_css_string(), "hsl(194,36,88)");
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
  /// assert_eq!(hsl_string, "hsl(0,0,0)");
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
  /// assert_eq!(hsl_string, "hsl(0,0,100)");
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
impl<'a> Into<ColorTuple> for &'a mut Hsl {
  fn into(self) -> ColorTuple {
    let Hsl { h, s, l, .. } = *self;
    (h, s, l)
  }
}
impl<'a> Into<ColorTuple> for &'a Hsl {
  fn into(self) -> ColorTuple {
    let Hsl { h, s, l, .. } = *self;
    (h, s, l)
  }
}

impl Into<ColorTuple> for Hsl {
  fn into(self) -> ColorTuple {
    self.as_ref().into()
  }
}

impl<'a> Into<ColorTupleA> for &'a Hsl {
  fn into(self) -> ColorTupleA {
    let Hsl { h, s, l, .. } = *self;
    (h, s, l, self.get_alpha())
  }
}
impl<'a> Into<ColorTupleA> for &'a mut Hsl {
  fn into(self) -> ColorTupleA {
    let Hsl { h, s, l, .. } = *self;
    (h, s, l, self.get_alpha())
  }
}

impl Into<ColorTupleA> for Hsl {
  fn into(self) -> ColorTupleA {
    self.as_ref().into()
  }
}
