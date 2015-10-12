
use std::ops::{Add, Mul, Sub};

pub type CustomFloat = f32;
const SIGNIFICAND: i32 = 24;

#[derive(Debug)]
pub struct Time(CustomFloat);

#[derive(Debug)]
pub struct Vector((CustomFloat, CustomFloat));

impl Vector {
  fn norm2(&self) -> CustomFloat {
    let &Vector((x1, x2)) = self;
    x1 * x1 + x2 * x2
  }
}

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

#[derive(Debug)]
pub struct Particle {
  x: Vector,
  v: Vector,
  r: CustomFloat,
  m: CustomFloat,
}

/// Computes solutions to the quadratic formula:
/// ( -b +/- sqrt(b^2 - 4ac) ) / 2a
/// The smaller solution is always on the left.
/// Returns None for imaginary results.
///
/// # Example
/// ```
/// use particles::quadratic_formula;
/// let (s1, s2) = quadratic_formula(1., 0., -1.).unwrap();
/// assert!((s1 + 1.).abs() < 1e-10);
/// assert!((s2 - 1.).abs() < 1e-10);
/// ```
pub fn quadratic_formula(a: CustomFloat, b: CustomFloat, c: CustomFloat) -> Option<(CustomFloat, CustomFloat)> {
  // account for floating error: we can't have degenerate solutions come up imaginary
  let b2 = b * b + (b / 2.0_f32.powi(-SIGNIFICAND + 1));
  let ac = 4. * a * c;

  if ( b2 < ac ) { None } // imaginary result
  else {
    let a2 = 2. * a;
    let fst = - b / a2;
    let snd = ((b2 - ac).sqrt() / a2).abs();
    Some((fst - snd, fst + snd))
  }
}

impl Particle {
  fn impact_time(&self, other: &Particle) -> Option<Time> {
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

    match quadratic_formula(a, b, c) {
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

  fn bounce(&self, other: &Particle) -> (Particle, Particle) {
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

trait Box<P> {
  fn create(initial: Vec<P>) -> Self;
  fn next_collision(&self) -> (&P, &P);
  fn update<'l>(&'l mut self, &p1: Particle, &p2: Particle) -> &'l mut Self;
}

fn main() {
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
  println!("p1: {:?}", p1);
  println!("delta: {:?}", &p1.x - &p2.x);
  println!("impact_time: {:?}", p1.impact_time(&p2));
}
