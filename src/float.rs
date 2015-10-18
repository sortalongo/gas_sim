use super::{CustomFloat};

use std::cmp::{Ord, Ordering};

#[derive(Debug, Clone, Copy)]
pub struct FloatOps(pub CustomFloat);

impl FloatOps {
  pub fn close(&self, other: &Self) -> bool {
    let (&FloatOps(this), &FloatOps(that)) = (self, other);
    (this - that).abs() < 1e-10
  }
}

impl PartialEq for FloatOps {
  fn eq(&self, other: &Self) -> bool {
    let (&FloatOps(this), &FloatOps(that)) = (self, other);
    this.eq(&that)
  }
  fn ne(&self, other: &Self) -> bool {
    let (&FloatOps(this), &FloatOps(that)) = (self, other);
    this.ne(&that)
  }
}

impl Eq for FloatOps { }

impl PartialOrd for FloatOps {
  fn partial_cmp(&self, other: &FloatOps) -> Option<Ordering> {
    let (&FloatOps(this), &FloatOps(that)) = (self, other);
    this.partial_cmp(&that)
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

