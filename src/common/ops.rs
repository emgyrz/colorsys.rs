use crate::ColorTuple;

type Op = Option<f64>;
type TupleA = (ColorTuple, Op);

pub fn add_sub_alpha(a1: &Op, a2: &Op, is_add: bool) -> Op {
  let has1 = a1.is_some();
  let has2 = a2.is_some();
  if !has1 && !has2 {
    return None;
  } else if has1 && has2 {
    let val1 = a1.unwrap();
    let val2 = a2.unwrap();
    let val = if is_add { val1 + val2 } else { val1 - val2 };
    return Some(val);
  }

  if has1 {
    *a1
  } else {
    *a2
  }
}

pub fn add_sub_tuples(t1: &ColorTuple, t2: &ColorTuple, is_add: bool) -> ColorTuple {
  let (x1, y1, z1) = t1;
  let (x2, y2, z2) = t2;

  if is_add {
    (x1 + x2, y1 + y2, z1 + z2)
  } else {
    (x1 - x2, y1 - y2, z1 - z2)
  }
}

pub fn add_sub_tuples_a(ta1: &TupleA, ta2: &TupleA, is_add: bool) -> TupleA {
  let (t1, a1) = ta1;
  let (t2, a2) = ta2;

  let t = add_sub_tuples(t1, t2, is_add);
  let a = add_sub_alpha(a1, a2, is_add);
  (t, a)
}
