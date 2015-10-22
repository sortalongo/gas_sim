extern crate particles;
extern crate rand;

use particles::*;
use rand::{StdRng};

fn main() {
  let max_particle = Particle {
    x: Vector((20., 20.)),
    v: Vector((1., 1.)),
    m: 1.,
    r: 1.
  };
  let min_particle = Particle {
    x: Vector((-20., -20.)),
    v: Vector((-1., -1.)),
    m: 1.,
    r: 1.
  };
  let mut rng = StdRng::new().unwrap();

  let particles = (1..10)
    .map( move |_| BoundedRand::rand(&mut rng, &min_particle, &max_particle))
    .collect::<Vec<_>>();

  let mut space = SpaceVec::new(particles);
  let mut i = 0;

  println!("starting: {:?}", space);
  while i < 10 && space.iterate() {
    println!("{}: {:?}", i, space);
    i += 1;
  }
  println!("ending");
}
