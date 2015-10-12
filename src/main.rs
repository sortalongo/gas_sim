extern crate particles;

use particles::*;

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
