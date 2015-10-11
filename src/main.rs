use std::ops::{Add, Mul, Sub};

type Float = f32;

struct Time(Float);

#[derive(Debug)]
struct Vector((Float, Float));

impl Mul for Vector {
  type Output = Float;
  fn mul(self, rhs: Vector) -> Float {
    let Vector(v1) = self;
    let Vector(v2) = rhs;
    v1.0 * v2.0 + v1.1 * v2.1
  }
}

impl Add for Vector {
  type Output = Vector;
  fn add(self, rhs: Vector) -> Vector {
    let Vector(v1) = self;
    let Vector(v2) = rhs;
    Vector((v1.0 + v2.0, v1.1 + v2.1))
  }
}

impl Sub for Vector {
  type Output = Vector;
  fn sub(self, rhs: Vector) -> Vector {
    let Vector(v1) = self;
    let Vector(v2) = rhs;
    Vector((v1.0 - v2.0, v1.1 - v2.1))
  }
}

#[derive(Debug)]
struct Particle {
  x: Vector,
  v: Vector,
  r: Float,
  m: Float,
}

impl Particle {
  fn tt_impact(&self, other: &Particle) -> Time {
    Time(0.0)
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
  println!("v1: {:?}", v1);
  println!("delta: {:?}", v1.x - v2.x);
}
