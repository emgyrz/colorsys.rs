#![macro_use]

macro_rules! into_for_some {
  ($into: ty, $some: ty, $sel: ident, $conv: block) => {
    impl<'a> Into<$into> for &'a $some {
      fn into($sel) -> $into {
        ($conv)
      }
    }

    impl<'a> Into<$into> for &'a mut $some {
      fn into($sel) -> $into {
        ($conv)
      }
    }

    impl Into<$into> for $some {
      fn into($sel) -> $into {
        $sel.as_ref().into()
      }
    }
  };
}
