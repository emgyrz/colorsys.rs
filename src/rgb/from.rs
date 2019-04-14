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
  fn from(hsl: &Hsl) -> Self {
    from_hsl(hsl)
  }
}
impl From<&mut Hsl> for Rgb {
  fn from(hsl: &mut Hsl) -> Self {
    from_hsl(hsl)
  }
}
impl From<Hsl> for Rgb {
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
