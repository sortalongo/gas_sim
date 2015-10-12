use super::{CustomFloat, SIGNIFICAND, Time};
use super::vector::Vector;

#[derive(Debug)]
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
///
/// # Examples
/// ```
/// use particles::particle::quadratic_formula;
/// let (s1, s2) = quadratic_formula(1., 0., -1.).unwrap();
/// assert!((s1 + 1.).abs() < 1e-10);
/// assert!((s2 - 1.).abs() < 1e-10);
/// ```
// TODO: make private, write unit tests for it
pub fn quadratic_formula(a: CustomFloat, b: CustomFloat, c: CustomFloat) -> Option<(CustomFloat, CustomFloat)> {
  // account for floating error: we can't have degenerate solutions come up imaginary
  let b2 = b * b + (b / 2.0_f32.powi(SIGNIFICAND - 1));
  println!("b2: {:?}", b2);
  let ac = 4. * a * c;
  println!("ac: {:?}", ac);

  if ( b2 < ac ) { None } // imaginary result
  else {
    let a2 = 2. * a;
    let fst = - b / a2;
    let snd = ((b2 - ac).sqrt() / a2).abs();
    Some((fst - snd, fst + snd))
  }
}

impl Particle {
/// Computes the next time the two given particles will impact each other.
/// Returns None if no such impact will occur.
///
/// # Panics
/// - if the two particles given overlap (i.e. they have fused together)
///
/// # Examples
/// ```
/// use particles::{Particle, Time, Vector};
/// let p1 = Particle {
///   x: Vector((-2., 0.)),
///   v: Vector((1., 0.)),
///   r: 1.,
///   m: 1.
/// };
/// let p2 = Particle {
///   x: Vector((2., 0.)),
///   v: Vector((-1., 0.)),
///   r: 1.,
///   m: 1.
/// };
/// let Time(t) = p1.impact_time(&p2).unwrap();
/// assert!((t - 1.).abs() < 1e-10);
/// ```
  pub fn impact_time(&self, other: &Particle) -> Option<Time> {
    // solves for t:
    // | self.x - self.x + (self.v - other.v) * t | = self.r + other.r
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
      Some((less, _)) if less >= 0. => Some(Time(less)),
      Some((_, more)) if more <= 0. => None,
      None => None, // particles will never impact
      Some(solns) => // less < 0 && more > 0 <=> particles overlap
        unreachable!(
          "impact_time found overlapping particles:\n{:?}\n{:?}\n{:?}",
          solns, self, other
        ),
    }
  }

  pub fn bounce(&self, other: &Particle) -> (Particle, Particle) {
    let v1 = Particle {
      x: Vector((1., 1.)),
      v: Vector((1., 1.)),
      r: 1.,
      m: 1.
    };
    let v2 = Particle {
      x: Vector((1., 1.)),
      v: Vector((1., 1.)),
      r: 1.,
      m: 1.
    };
    (v1, v2)
  }
}

