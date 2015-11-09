use super::{custom_float, CustomFloat, Time};
use super::vector::Vector;

#[derive(Clone, Debug)]
pub struct Particle {
  pub x: Vector,
  pub v: Vector,
  pub r: CustomFloat,
  pub m: CustomFloat,
}

/// Computes solutions to the quadratic formula:
/// ( -b +/- sqrt(b^2 - 4ac) ) / 2a
/// The smaller solution is always on the left.
/// Returns None for imaginary results.
fn quadratic_formula(a: CustomFloat, b: CustomFloat, c: CustomFloat) -> Option<(CustomFloat, CustomFloat)> {
  // account for rounding error:
  // in the case of b = 4ac, rounding error may cause b < 4ac.
  // So, we increment the last bit in the mantissa by one
  let b2 = b * b + (b / (2.0 as CustomFloat).powi((custom_float::MANTISSA_DIGITS - 1) as i32));
  let ac = 4. * a * c;

  if b2 < ac { None } // imaginary result
  else {
    let a2 = 2. * a;
    let fst = - b / a2;
    let snd = ((b2 - ac).sqrt() / a2).abs();
    Some((fst - snd, fst + snd))
  }
}

impl Particle {
  pub fn overlaps(&self, other: &Particle) -> bool {
    let d = (&self.x - &other.x).norm();
    let r = self.r + other.r;
    d < r - 1e-5
  }

  /// Computes the next time the two given particles will impact each other.
  /// Returns None if no such impact will occur.
  ///
  /// # Panics
  /// - if the two particles given overlap (i.e. they have fused together)
  pub fn impact_time(&self, other: &Particle) -> Option<Time> {
    // solves for t:
    // | self.x - other.x + (self.v - other.v) * t | = self.r + other.r
    let dv = &(&self.v - &other.v);
    let dx = &(&self.x - &other.x);
    let sr = self.r + other.r;

    // quadratic formula for t:
    // |dv|^2 t^2 + 2 (dx * dv) t + |dx|^2 - sr^2 = 0
    let a = dv.norm2();
    let b = 2. * ( dx * dv );
    let c = dx.norm2() - sr.powi(2);

    let s = quadratic_formula(a, b, c);
    match s {
      None => None, // particles will never impact
      Some((_, more)) if more <= 0. + 1e-5 => None,
      Some((less, more)) => {
        assert!(
          !self.overlaps(other),
          "impact_time found overlapping particles:\n\
          distance: {:?}\n\
          self: {:?}\n\
          other: {:?}\n",
          (&self.x - &other.x).norm(), self, other
        );
        assert!(
          !(less < 0.),
          "impact_time found negative solution to quadratic formula:\n\
          solution: {:?}\n\
          distance: {:?}\n\
          self: {:?}\n\
          other: {:?}\n",
          (less, more), (&self.x - &other.x).norm(), self, other
        );
        Some(Time(less))
      }
    }
  }

  /// Returns new particles after a collision.
  /// Assumes that the particles are tangent to each other.
  /// The first particle returned corresponds to self.
  ///
  /// # Panics
  /// - if the two particles are not tangent (or within 1e-5 units)
  pub fn bounce(&self, other: &Particle) -> (Particle, Particle) {
    let r_t = self.r + other.r;
    let dx = &self.x - &other.x;
    // only works for particles in contact
    assert!(
      (dx.norm() - r_t).abs() < 1e-5,
      "bounce was given non-tangent particles:\n\
      distance: {:?}\n\
      self: {:?}\n\
      other: {:?}\n",
      (&self.x - &other.x).norm(), self, other
    );

    let dv = &self.v - &other.v;
    let m_r = self.m * other.m / (self.m + other.m);

    // dp = 2 m1 m2 / (m1 + m2) (dv . \hat{dx}) \hat{dx}
    let dp = dx.scale(2. * m_r * (&dv * &dx) / dx.norm2());
    let v1 = &self.v - &dp.scale(1. / self.m);
    let v2 = &other.v + &dp.scale(1. / other.m);

    let p1 = Particle { v: v1, .. self.clone() };
    let p2 = Particle { v: v2, .. other.clone() };
    (p1, p2)
  }

  pub fn evolve(&self, t: Time) -> Particle {
    let Time(t_) = t;
    Particle { x: &self.x + &self.v.scale(t_), .. self.clone() }
  }

  pub fn after_bounce(&self, other: &Particle, t: Time) -> (Particle, Particle) {
    let (prebounce1, prebounce2) = (self.evolve(t), other.evolve(t));
    prebounce1.bounce(&prebounce2)
  }
}

impl PartialEq for Particle {
  fn eq(&self, other: &Particle) -> bool {
    self.r.eq(&other.r) &&
    self.m.eq(&other.m) &&
    self.x.eq(&other.x) &&
    self.v.eq(&other.v)
  }

  fn ne(&self, other: &Particle) -> bool {
    self.r.ne(&other.r) ||
    self.m.ne(&other.m) ||
    self.x.ne(&other.x) ||
    self.v.ne(&other.v)
  }
}

impl Eq for Particle {}

#[cfg(test)]
mod tests {
  use super::super::{FloatOps, Particle, Time, Vector};

  #[test]
  fn quadratic_formula_simple() {
    use super::quadratic_formula;
    let (s1, s2) = quadratic_formula(1., 0., -1.).unwrap();
    assert!(FloatOps(s1).close(&FloatOps(-1.)));
    assert!(FloatOps(s2).close(&FloatOps(1.)));
  }

  #[test]
  fn bounce_symmetrical_particles() {
    let p1 = Particle {
      x: Vector((-1., 0.)),
      v: Vector((1., 0.)),
      r: 1.,
      m: 1.
    };
    let p2 = Particle {
      x: Vector((1., 0.)),
      v: Vector((-1., 0.)),
      r: 1.,
      m: 1.
    };
    let (p1_, p2_) = p1.bounce(&p2);
    assert!((&p1_.v - &Vector((-1., 0.))).norm() < 1e-10);
    assert!((&p2_.v - &Vector((1., 0.))).norm() < 1e-10);
  }

  #[test]
  fn impact_time_symmetrical_partices() {
    let p1 = Particle {
      x: Vector((-2., 0.)),
      v: Vector((1., 0.)),
      r: 1.,
      m: 1.
    };
    let p2 = Particle {
      x: Vector((2., 0.)),
      v: Vector((-1., 0.)),
      r: 1.,
      m: 1.
    };
    let Time(t) = p1.impact_time(&p2).unwrap();
    assert!((t - 1.).abs() < 1e-10);
  }
}
