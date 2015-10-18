use super::{CustomFloat};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Vector(pub (CustomFloat, CustomFloat));

impl Vector {
  pub fn norm(&self) -> CustomFloat {
    self.norm2().sqrt()
  }
  pub fn norm2(&self) -> CustomFloat {
    let &Vector((x1, x2)) = self;
    x1 * x1 + x2 * x2
  }

  pub fn scale(&self, a: CustomFloat) -> Vector {
    let &Vector((x1, x2)) = self;
    Vector((a * x1, a * x2))
  }
}

impl PartialEq for Vector {
  fn eq(&self, other: &Vector) -> bool {
    let &Vector((x1, x2)) = self;
    let &Vector((y1, y2)) = other;
    x1.eq(&y1) && x2.eq(&y2)
  }

  fn ne(&self, other: &Vector) -> bool {
    let &Vector((x1, x2)) = self;
    let &Vector((y1, y2)) = other;
    x1.ne(&y1) || x2.ne(&y2)
  }
}

impl Eq for Vector { }

impl<'l> Mul for &'l Vector {
  type Output = CustomFloat;
  fn mul(self, rhs: &Vector) -> CustomFloat {
    let &Vector(v1) = self;
    let &Vector(v2) = rhs;
    v1.0 * v2.0 + v1.1 * v2.1
  }
}

impl<'l> Add for &'l Vector {
  type Output = Vector;
  fn add(self, rhs: &Vector) -> Vector {
    let &Vector(v1) = self;
    let &Vector(v2) = rhs;
    Vector((v1.0 + v2.0, v1.1 + v2.1))
  }
}

impl<'l> Sub for &'l Vector {
  type Output = Vector;
  fn sub(self, rhs: &Vector) -> Vector {
    let &Vector(v1) = self;
    let &Vector(v2) = rhs;
    Vector((v1.0 - v2.0, v1.1 - v2.1))
  }
}

