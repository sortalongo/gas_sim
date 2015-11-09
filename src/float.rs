use super::{CustomFloat};

use std::cmp::{Ord, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FloatOps(pub CustomFloat);

impl Eq for FloatOps { }

impl FloatOps {
  pub fn close(&self, other: &Self) -> bool {
    let (&FloatOps(this), &FloatOps(that)) = (self, other);
    (this - that).abs() < 1e-8
  }
}

impl Ord for FloatOps {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.partial_cmp(other) {
      Some(ord) => ord,
      None => panic!("Unable to compare floats: {:?} and {:?}", self, other)
    }
  }
}

