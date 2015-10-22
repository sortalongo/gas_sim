use rand::{Rng};

use super::{CustomFloat, Particle, Vector};

pub trait BoundedRand {
  /// pseudo-extends the Rand typeclass with the ability to specify bounds
  /// for the generated type. Bounds are assumed to be closed.
  fn rand <R: Rng>(rng: &mut R, lower: &Self, upper: &Self) -> Self;
}

impl BoundedRand for CustomFloat {
  fn rand <R: Rng>(rng: &mut R, lower: &CustomFloat, upper: &CustomFloat) -> CustomFloat {
    if (upper - lower).eq(&0.) {
      *lower
    } else {
      let gen = rng.gen::<CustomFloat>();
      gen * (upper-lower) + lower
    }
  }
}


impl BoundedRand for Vector {
  fn rand <R: Rng>(rng: &mut R, lower: &Vector, upper: &Vector) -> Vector {
    let &Vector((min1, min2)) = lower;
    let &Vector((max1, max2)) = upper;

    Vector((
      BoundedRand::rand(rng, &min1, &max1),
      BoundedRand::rand(rng, &min2, &max2)
    ))
  }
}

impl BoundedRand for Particle {
  fn rand <R: Rng>(rng: &mut R, lower: &Particle, upper: &Particle) -> Particle {
    Particle {
      x: BoundedRand::rand(rng, &lower.x, &upper.x),
      v: BoundedRand::rand(rng, &lower.v, &upper.v),
      r: BoundedRand::rand(rng, &lower.r, &upper.r),
      m: BoundedRand::rand(rng, &lower.m, &upper.m)
    }
  }
}

#[cfg(test)]
extern crate quickcheck;

#[cfg(test)]
mod tests {
  use rand::thread_rng;
  use super::quickcheck::quickcheck;

  use super::*;
  use ::{CustomFloat, Vector};

  #[test]
  fn custom_float_is_bounded() {
    fn prop(bound1: CustomFloat, bound2: CustomFloat) -> bool {
      let mut rng = thread_rng();
      let min = bound1.min(bound2);
      let max = bound1.max(bound2);

      let mut correct = true;
      for _ in 1..50 {
        let gen = BoundedRand::rand(&mut rng, &min, &max);
        correct |= gen <= max;
        correct |= gen >= min;
      }
      correct
    }

    quickcheck(prop as fn(CustomFloat, CustomFloat) -> bool);
  }

  #[test]
  fn custom_float_can_be_constant() {
    let mut rng = thread_rng();
    let min = 1.;
    let max = 1.;

    for _ in 1..50 {
      let gen = BoundedRand::rand(&mut rng, &min, &max);
      assert!(gen <= max);
      assert!(gen >= min);
    }
  }

  #[test]
  fn vector_is_bounded() {
    let mut rng = thread_rng();
    let min = Vector((-2e4, -8.77e7));
    let max = Vector((5.6e8, 3.2e3));

    for _ in 1..50 {
      let Vector(gen) = BoundedRand::rand(&mut rng, &min, &max);
      assert!(gen.0 <= (max.0).0);
      assert!(gen.1 <= (max.0).1);
      assert!(gen.0 >= (min.0).0);
      assert!(gen.1 >= (min.0).1);
    }
  }
}
